use std::net::SocketAddr;
use std::io::{Read, Write};

use param::{Credential, Host, Limit};
use tcore::net::TcpStream;

use tserv::{Service, NewService};

///
/// Session are struct that holds TcpStream.
///
pub struct Session {
    socket: TcpStream,
    host: Host,
    credential: Credential,
    limit: Limit
}

// impl Service for Session { }

// impl NewService for Session { }

// static LOCAL_ADDR : &'static SocketAddr::V4 = SocketAddr::V4::new((127, 0, 0, 1), 5672);

// impl Session {

//     pub fn with_default() -> std::io::Result<Session> {
//         let socket = TcpStream::connect(LOCAL_ADDR).expect("can't create connection");

//         Session {
//             socket: socket,
//             host: Host::default(),
//             credential: Credential::default(),
//             limit: Limit::default()
//         }
//     }

//     pub fn create(host: Host, credential: Credential, limit: Option<Limit>) -> std::io::Result<Session> {

//         let socket = TcpStream::connect(LOCAL_ADDR).expect("can't create connection");

//         Session {
//             socket: socket,
//             host: host,
//             credential: credential,
//             limit: limit.unwrap_or_default()
//         }

//     }
// }

// // impl Read for Session {

// // }

// // impl Write for Session {

// // }

// ///
// /// Current default will use
// ///
// impl Default for Session {
//     fn default() -> Session {
//         Session {
//             socket: core::net::
//         }
//     }
// }

// impl Drop for Session {
//     fn drop(&self) {
//         self.socket.shutdown().ok().expect("can't shutdown the socket");
//     }
// }


// pub struct SessionPool {}

// impl Pool for SessionPool {
//     type Instance = Connection;
//     type Error = ();
// }

// impl Default for SessionPool {
// }
