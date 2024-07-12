use std::net::{IpAddr, SocketAddr, UdpSocket};

mod commands {
    pub mod player_info;
}

pub struct ServerConnection {
    socket: UdpSocket,
}

impl ServerConnection {
    pub fn connect(server_address: ServerAddress) -> Result<ServerConnection, std::io::Error> {
        let socket_addr = SocketAddr::new(server_address.address, server_address.port);
        let socket = UdpSocket::bind(socket_addr)?;

        return Ok(ServerConnection { socket });
    }
}

pub struct ServerAddress {
    address: IpAddr,
    port: u16,
}
