use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    pub lvl: String,
}

impl TracingConfig {
    pub fn init(&self) {
        tracing_subscriber::fmt()
            // .with_span_events(
            //     tracing_subscriber::fmt::format::FmtSpan::ENTER
            //         | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            // )
            .with_env_filter(self.lvl.clone())
            .init();
    }
}
