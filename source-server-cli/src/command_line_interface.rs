use clap::Parser;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum QueryCommand {
    Players,
}

#[derive(Parser)]
pub struct Cli {
    pub ip: String,
    pub port: u16,
    pub command: QueryCommand,
}
