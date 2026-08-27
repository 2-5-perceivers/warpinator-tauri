use crate::tracing::LogLevel;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[uniffi::export]
pub fn set_tracing_subscriber(tag: String, max_level: LogLevel) {
    let filter = match max_level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
    };

    let layer = tracing_android::layer(tag.as_str()).unwrap();
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
}
