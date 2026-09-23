use abi_stable::std_types::RStr;
use bridge_api::{BridgeHostLogSink, RLogLevel};
use std::sync::{Mutex, OnceLock};

static HOST_LOG_SINK: Mutex<Option<BridgeHostLogSink>> = Mutex::new(None);
static DRC_LOG_ENABLED: OnceLock<bool> = OnceLock::new();

pub(crate) extern "C" fn register_host_log_sink(sink: usize) {
    {
        let mut slot = HOST_LOG_SINK
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        *slot = if sink == 0 {
            None
        } else {
            Some(unsafe { std::mem::transmute::<usize, BridgeHostLogSink>(sink) })
        };
    }
    if sink != 0 {
        install_host_logger();
    }
}

/// Forwards this plugin's own `log` records to the host.
///
/// A plugin carries its own copy of the `log` crate, so the logger the host
/// installed never sees a `log::warn!` made in here: with none installed on
/// this side, the maximum level stays off and every record - the DTS
/// pipeline's reports that a `DTS:X` extension could not be read, among them -
/// is discarded before it is formatted. This one sends them the same way
/// `bridge_diag_log` does: to the host sink, or to stderr without one.
///
/// Only this crate's records, though - see [`is_bridge_target`].
struct HostLogger;

/// Whether a record was made by this crate rather than by a decoder it links.
///
/// The decoders log for their own debugging, and at whatever level they are
/// handed: the E-AC-3 decoders trace every block they parse at the bridge's
/// failure level, `Error` unless strict, which forwarded came to over 130,000
/// ERROR lines for twenty seconds of Dolby Digital Plus Atmos. Nothing a
/// listener needs is lost by leaving that where it was before this logger
/// existed: every parse, extract and decode failure is reported again here, by
/// the pipeline that met it, and the DTS:X extension reports a stream can
/// repeat every frame are rate limited.
fn is_bridge_target(target: &str) -> bool {
    target == "harletty_bridge" || target.starts_with("harletty_bridge::")
}

impl log::Log for HostLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level() && is_bridge_target(metadata.target())
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            bridge_external_log(record.level(), record.target(), &record.args().to_string());
        }
    }

    fn flush(&self) {}
}

static HOST_LOGGER: HostLogger = HostLogger;

/// Install [`HostLogger`] once, at the level the host's engine logs at: a bare
/// level in `RUST_LOG` (as the engine reads it), else `info`. A binary that
/// links this crate and has already installed a logger of its own keeps it.
fn install_host_logger() {
    if log::set_logger(&HOST_LOGGER).is_ok() {
        let level = std::env::var("RUST_LOG")
            .ok()
            .and_then(|value| value.trim().parse::<log::LevelFilter>().ok())
            .unwrap_or(log::LevelFilter::Info);
        log::set_max_level(level);
    }
}

pub(crate) fn bridge_diag_log(level: log::Level, message: &str) {
    bridge_external_log(level, "harletty-bridge::diag", message);
}

pub(crate) fn drc_diag_log_enabled() -> bool {
    *DRC_LOG_ENABLED.get_or_init(|| {
        std::env::var_os("HARLETTY_LOG_DRC")
            .map(|value| value != "0")
            .unwrap_or(false)
    })
}

pub(crate) fn bridge_external_log(level: log::Level, target: &str, message: &str) {
    let trimmed = message.trim_end_matches('\n');
    let sink = {
        let slot = HOST_LOG_SINK
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        *slot
    };
    if let Some(callback) = sink {
        callback(
            encode_log_level(level),
            RStr::from(target),
            RStr::from(trimmed),
        );
    } else {
        eprintln!("{trimmed}");
    }
}

pub(crate) fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic during frame processing".to_string()
    }
}

fn encode_log_level(level: log::Level) -> RLogLevel {
    match level {
        log::Level::Error => RLogLevel::Error,
        log::Level::Warn => RLogLevel::Warn,
        log::Level::Info => RLogLevel::Info,
        log::Level::Debug => RLogLevel::Debug,
        log::Level::Trace => RLogLevel::Trace,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SEEN: Mutex<Vec<(RLogLevel, String, String)>> = Mutex::new(Vec::new());

    extern "C" fn capture(level: RLogLevel, target: RStr<'_>, message: RStr<'_>) {
        SEEN.lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .push((
                level,
                target.as_str().to_owned(),
                message.as_str().to_owned(),
            ));
    }

    #[test]
    fn the_bridges_own_records_reach_the_host_sink_and_a_decoders_do_not() {
        register_host_log_sink(capture as BridgeHostLogSink as usize);
        log::warn!("host sink probe {}", 42);
        // What the E-AC-3 decoder writes for every block it parses, at the
        // failure level the bridge hands it.
        log::error!(target: "starmine_ad::eac3dec::aux", "decoder probe {}", 7);
        register_host_log_sink(0);

        let seen = SEEN.lock().unwrap_or_else(|poison| poison.into_inner());
        assert!(
            seen.iter().any(|(level, target, message)| {
                *level == RLogLevel::Warn
                    && target.starts_with("harletty_bridge")
                    && message == "host sink probe 42"
            }),
            "{seen:?}"
        );
        assert!(
            !seen
                .iter()
                .any(|(_, target, message)| target.starts_with("starmine_ad")
                    || message == "decoder probe 7"),
            "{seen:?}"
        );
    }
}
