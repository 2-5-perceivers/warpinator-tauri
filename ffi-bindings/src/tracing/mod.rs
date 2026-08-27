#[cfg(feature = "tracing_callbacks")]
mod callbacks;
#[cfg(feature = "tracing_callbacks")]
pub use callbacks::*;

#[cfg(feature = "tracing_android")]
mod android;
#[cfg(feature = "tracing_android")]
pub use android::*;

#[cfg(all(feature = "tracing_android", feature = "tracing_callbacks"))]
compile_error!(
    "Features `tracing_android` and `tracing_callbacks` are mutually exclusive. Enable only one at a time."
);

#[derive(uniffi::Enum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
