extern crate tokio_io as tio;
extern crate tokio_core as tcore;
extern crate tokio_proto as tproto;

extern crate bytes;

use std::net::SocketAddr;


#[test]
fn test_connect() {
    let mut stream = tcore::net::TcpStream::connect(("127.0.0.1", 5672)).unwrap();

}
