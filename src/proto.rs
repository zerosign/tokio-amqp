use tproto::BindClient;

use tcore::reactor::{Core, Handle, Remote};
use tio::{AsyncRead, AsyncWrite};
use tio::codec::Framed;
use tproto::pipeline::{ClientService, ClientProto};

use tserv::{Service, NewService};
use bytes::Bytes;
use std::io;

use codec::AMQPCodec;

pub struct AMQPProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for AMQPProto {
    type Request = Bytes;
    type Response = Bytes;

    type Transport = Framed<T, AMQPCodec>;
    // type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(AMQPCodec))
    }
}
