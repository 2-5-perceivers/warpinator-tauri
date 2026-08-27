use crate::tracing::LogLevel;
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[uniffi::export]
pub fn set_log_listener(listener: Box<dyn LogListener>, max_level: LogLevel) {
    let filter = match max_level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
    };

    let layer = CallbacksLayer {
        listener: Arc::new(listener),
    };
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
}

#[uniffi::export(callback_interface)]
pub trait LogListener: Send + Sync {
    fn on_log(&self, level: LogLevel, message: String);
}

struct CallbacksLayer {
    listener: Arc<Box<dyn LogListener>>,
}

impl<S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>> Layer<S>
    for CallbacksLayer
{
    fn on_event(&self, event: &tracing::Event, ctx: tracing_subscriber::layer::Context<S>) {
        let level = match *event.metadata().level() {
            tracing::Level::TRACE => LogLevel::Trace,
            tracing::Level::DEBUG => LogLevel::Debug,
            tracing::Level::INFO => LogLevel::Info,
            tracing::Level::WARN => LogLevel::Warn,
            tracing::Level::ERROR => LogLevel::Error,
        };

        // build span chain e.g. "outer::inner::current"
        let span_chain = ctx
            .event_scope(event)
            .map(|scope| {
                scope
                    .from_root()
                    .map(|span| span.name())
                    .collect::<Vec<_>>()
                    .join("::")
            })
            .unwrap_or_default();

        let mut message = String::new();
        event.record(&mut StringVisitor(&mut message));

        let full_message = if span_chain.is_empty() {
            message
        } else {
            format!("[{}] {}", span_chain, message)
        };

        self.listener.on_log(level, full_message);
    }
}

struct StringVisitor<'a>(&'a mut String);
impl tracing::field::Visit for StringVisitor<'_> {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.0.push_str(value);
        }
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0.push_str(&format!("{:?}", value));
        }
    }
}
