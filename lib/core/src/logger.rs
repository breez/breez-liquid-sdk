use std::fs::OpenOptions;
use std::io::Write;

use anyhow::{anyhow, Result};
use chrono::Local;
use log::{LevelFilter, Metadata, Record};

use crate::model::LogEntry;

pub(crate) struct GlobalSdkLogger {
    /// SDK internal logger, which logs to file
    pub(crate) logger: env_logger::Logger,
    /// Optional external log listener, that can receive a stream of log statements
    pub(crate) log_listener: Option<Box<dyn log::Log>>,
}
impl log::Log for GlobalSdkLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.logger.log(record);

            if let Some(s) = &self.log_listener.as_ref() {
                if s.enabled(record.metadata()) {
                    s.log(record);
                }
            }
        }
    }

    fn flush(&self) {}
}

pub(super) fn init_logging(log_dir: &str, app_logger: Option<Box<dyn log::Log>>) -> Result<()> {
    let target_log_file = Box::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{log_dir}/sdk.log"))
            .map_err(|e| anyhow!("Can't create log file: {e}"))?,
    );
    let logger = env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target_log_file))
        .parse_filters(
            r#"
                debug,
                breez_sdk_liquid=debug,
                breez_sdk_liquid::swapper::boltz_status_stream=info,
                electrum_client::raw_client=warn,
                lwk_wollet=info,
                rustls=warn,
                rustyline=warn,
                ureq=info,
                tungstenite=warn
            "#,
        )
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .build();

    let global_logger = GlobalSdkLogger {
        logger,
        log_listener: app_logger,
    };

    log::set_boxed_logger(Box::new(global_logger))
        .map_err(|e| anyhow!("Failed to set global logger: {e}"))?;
    log::set_max_level(LevelFilter::Trace);

    Ok(())
}

pub trait Logger: Send + Sync {
    fn log(&self, l: LogEntry);
}
