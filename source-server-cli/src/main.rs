use std::process::ExitCode;

use clap::Parser;
use source_query_protocol::ServerAddress;

mod command_line_interface;
mod source_query_protocol;

fn main() -> ExitCode {
    let arguments = command_line_interface::Cli::parse();
    println!(
        "{}, {}, {:?}",
        arguments.ip, arguments.port, arguments.command
    );

    let server_connection = source_query_protocol::ServerConnection::connect(
        ServerAddress::from_ipstr(arguments.ip, arguments.port),
    );

    if !server_connection.is_ok() {
        println!("Failed to connect to server");
        // print what went wrong
        println!("{}", server_connection.err().unwrap());
        return ExitCode::FAILURE;
    }

    println!("Connected to server");

    match arguments.command {
        command_line_interface::QueryCommand::Players => {
            println!("Querying players");
        }
    }

    return ExitCode::SUCCESS;
}
