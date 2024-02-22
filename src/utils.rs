use clap_verbosity_flag::{Level, Verbosity};

pub fn log(message: String, verbosity: &Verbosity, min_level: Level) {
    if verbosity.log_level().unwrap_or(Level::Info) >= min_level {
        println!("{message}");
    }
}
