use {fern, log};

pub use log::LevelFilter::*;

struct JsLog;

impl log::Log for JsLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        let message = format!("{}", record.args());
        js! {
            console.log(@{message});
        }
    }
    fn flush(&self) {}
}

pub fn setup_logging(verbosity: log::LevelFilter) {
    fern::Dispatch::new()
        .level(verbosity)
        .format(|out, message, record| {
            out.finish(format_args!(
                "({}) {}: {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(Box::new(JsLog) as Box<log::Log>)
        .apply()
        .expect("expected fern to initialize");
}
