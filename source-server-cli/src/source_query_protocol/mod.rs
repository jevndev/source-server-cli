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
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(socket_addr)?;

        return Ok(ServerConnection { socket });
    }
}

pub struct ServerAddress {
    address: IpAddr,
    port: u16,
}

impl ServerAddress {
    pub fn from_ipstr(ip: String, port: u16) -> ServerAddress {
        let ip_addr: IpAddr = ip.parse().expect("Invalid IP address format");
        return ServerAddress {
            address: ip_addr,
            port: port,
        };
    }
}
