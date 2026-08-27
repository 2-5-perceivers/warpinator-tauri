use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct ProtocolFeatures: u32 {
        const NONE = 0;
        #[cfg(feature = "messaging")]
        const MESSAGE_SUPPORT = 0b0001;
    }
}

impl Default for ProtocolFeatures {
    fn default() -> Self {
        let features = ProtocolFeatures::NONE;
        #[cfg(feature = "messaging")]
        let features = features | ProtocolFeatures::MESSAGE_SUPPORT;
        features
    }
}
