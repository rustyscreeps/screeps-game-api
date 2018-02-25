use {fern, log};

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

pub fn setup_logging(verbosity: u32) {
    let level = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    fern::Dispatch::new()
        .level(level)
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
        .unwrap();
}
