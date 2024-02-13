use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The duration after which the screen shall be displayed (in minutes)
    #[arg(short, long, default_value_t = 1)]
    pub timer: i32,
    /// The duration without events after which the user is considered afk (in minutes)
    #[arg(short, long, default_value_t = 1)]
    pub inactive: i32,
}