use settings::Settings;

pub struct SentryTracing {
    _guard: Option<()>,
}

impl SentryTracing {
    pub fn init(settings: &Settings, service: &str) -> Self {
        tracing_subscriber::fmt()
            .with_target(false)
            .with_max_level(
                settings
                    .log
                    .level
                    .parse::<tracing::Level>()
                    .expect("Failed to parse log level"),
            )
            .init();

        Self { _guard: None }
    }
}

pub fn error_with_context<E: std::error::Error + ?Sized>(
    message: &str,
    error: &E,
    context: &[(&str, &str)],
) {
    tracing::error!("{}: {}", message, error);
}

pub fn info_with_context(message: &str, context: &[(&str, &str)]) {
    let mut fields = vec![];
    for (key, value) in context {
        fields.push(format!("{}={}", key, value));
    }
    if fields.is_empty() {
        tracing::info!("{}", message);
    } else {
        tracing::info!("{} {}", message, fields.join(" "));
    }
}

pub fn warn_with_context(message: &str, context: &[(&str, &str)]) {
    let mut fields = vec![];
    for (key, value) in context {
        fields.push(format!("{}={}", key, value));
    }
    if fields.is_empty() {
        tracing::warn!("{}", message);
    } else {
        tracing::warn!("{} {}", message, fields.join(" "));
    }
}

#[macro_export]
macro_rules! warn_ctx {
    ($message:expr, $($var:ident),* $(,)?) => {
        {
            let context = &[
                $(
                    (stringify!($var), &$var.to_string()),
                )*
            ];
            $crate::warn_with_context($message, context);
        }
    };
}

pub fn error<E: std::error::Error + ?Sized>(message: &str, error: &E) {
    tracing::error!("{}: {}", message, error);
}
