use std::net::IpAddr;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use crypto_secretbox::aead::{Aead, KeyInit};
use crypto_secretbox::{AeadCore, Key, Nonce, XSalsa20Poly1305};
use rsa::RsaPrivateKey;
use rsa::pkcs1::der::asn1::OctetString;
use rsa::pkcs8::EncodePrivateKey;
use rsa::rand_core::OsRng;
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::instrument;
use x509_cert::builder::{Builder, CertificateBuilder, Profile};
use x509_cert::der::EncodePem;
use x509_cert::ext::pkix::SubjectAltName;
use x509_cert::ext::pkix::name::GeneralName;
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::spki::SubjectPublicKeyInfoOwned;
use x509_cert::time::{Time, Validity};

#[derive(Error, Debug)]
pub enum CertUnboxError {
    #[error("Boxed cert too short")]
    BoxTooShort,
    #[error("Decryption failed — wrong group code?")]
    DecryptionFailed,
}

#[derive(Error, Debug)]
pub enum AuthenticatorError {
    #[error("Private Key generation failed: {0}")]
    PrivateKeyError(#[from] rsa::Error),
    #[error("Public Key generation failed: {0}")]
    PublicKeyError(#[from] x509_cert::spki::Error),
    #[error(transparent)]
    DerError(#[from] x509_cert::der::Error),
    #[error("Certificate generation failed: {0}")]
    CertificateError(#[from] x509_cert::builder::Error),
    #[error("PEM error")]
    PemError(#[from] rsa::pkcs8::Error),
}

type GeneratedCert = (Vec<u8>, Vec<u8>); // (cert_pem, private_key_pem)

#[derive(Debug)]
pub struct Authenticator {
    /// The group code used to derive the SecretBox key
    group_code: String,

    /// PEM-encoded certificate bytes — what gets boxed and sent to peers
    cert_pem: Vec<u8>,

    /// PEM-encoded private key — used for TLS
    private_key_pem: Vec<u8>,
}

impl Authenticator {
    pub fn new(
        group_code: String,
        hostname: &str,
        local_ip: IpAddr,
    ) -> Result<Self, AuthenticatorError> {
        let (cert_pem, private_key_pem) = Self::generate_cert(hostname, local_ip)?;

        Ok(Self { group_code, cert_pem, private_key_pem })
    }

    /// Returns the cert boxed with the group code key, ready to send to peers.
    /// Returns None if it fails to encrypt the cert_pem
    #[instrument(skip_all, level = "trace")]
    pub fn box_cert(&self) -> Result<Vec<u8>, crypto_secretbox::Error> {
        let key = self.derive_key();
        let cipher = XSalsa20Poly1305::new(Key::from_slice(&key));
        let nonce = XSalsa20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher.encrypt(&nonce, self.cert_pem.as_slice())?;

        // Layout: [nonce (24 bytes) | ciphertext]
        let mut result = Vec::with_capacity(24 + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypts a boxed cert received from a peer
    #[instrument(skip_all, level = "trace", err)]
    pub fn unbox_cert(&self, boxed: &[u8]) -> Result<Vec<u8>, CertUnboxError> {
        if boxed.len() < 24 {
            return Err(CertUnboxError::BoxTooShort);
        }

        let key = self.derive_key();
        let cipher = XSalsa20Poly1305::new(Key::from_slice(&key));

        let nonce = Nonce::from_slice(&boxed[..24]);
        let ciphertext = &boxed[24..];

        let cert =
            cipher.decrypt(nonce, ciphertext).map_err(|_| CertUnboxError::DecryptionFailed)?;

        Ok(cert)
    }

    /// The PEM cert, for use in TLS
    pub fn cert_pem(&self) -> &[u8] {
        &self.cert_pem
    }

    /// The private key PEM, for use in TLS
    pub fn private_key_pem(&self) -> &[u8] {
        &self.private_key_pem
    }

    fn derive_key(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.group_code.as_bytes());
        hasher.finalize().to_vec()
    }

    #[instrument(level = "debug", err)]
    fn generate_cert(
        hostname: &str,
        local_ip: IpAddr,
    ) -> Result<GeneratedCert, AuthenticatorError> {
        let mut rng = OsRng;

        // Generate RSA 2048 keypair
        let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
        let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key.clone());

        let sanitized_hostname =
            hostname.replace(|c: char| !c.is_ascii_alphanumeric(), "").trim().to_string();
        let hostname =
            if sanitized_hostname.is_empty() { "warpinator" } else { &sanitized_hostname };

        let subject = Name::from_str(&format!("CN={}", hostname))?;

        let not_before_sys = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
        let not_after_sys = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 30);
        let not_before = Time::try_from(not_before_sys)?;
        let not_after = Time::try_from(not_after_sys)?;

        let validity = Validity { not_before, not_after };

        let serial = SerialNumber::from(rand::random::<u64>());

        let spki = SubjectPublicKeyInfoOwned::from_key(private_key.to_public_key())?;

        let mut builder = CertificateBuilder::new(
            Profile::Leaf {
                issuer: subject.clone(),
                enable_key_agreement: false,
                enable_key_encipherment: true,
            },
            serial,
            validity,
            subject,
            spki,
            &signing_key,
        )?;

        // Add SAN extension with IP
        let san = SubjectAltName(vec![match local_ip {
            IpAddr::V4(ip) => GeneralName::IpAddress(OctetString::new(ip.octets().as_slice())?),
            IpAddr::V6(ip) => GeneralName::IpAddress(OctetString::new(ip.octets().as_slice())?),
        }]);
        builder.add_extension(&san)?;

        let cert = builder.build()?;

        let cert_pem = cert.to_pem(Default::default())?.into_bytes();
        let private_key_pem = private_key.to_pkcs8_pem(Default::default())?.as_bytes().to_vec();

        Ok((cert_pem, private_key_pem))
    }
}
