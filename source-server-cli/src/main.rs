use clap::Parser;

mod command_line_interface;

fn main() {
    let arguments = command_line_interface::Cli::parse();
    println!(
        "{}, {}, {:?}",
        arguments.ip, arguments.port, arguments.command
    )
}
