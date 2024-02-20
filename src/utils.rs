use clap_verbosity_flag::Level;

pub fn log(message: String, level: Option<Level>, min_level: Level) {
    if level.unwrap_or(Level::Info) >= min_level {
        println!("{message}");
    }
}
