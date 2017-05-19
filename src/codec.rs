use tio::codec::{Framed, Encoder, Decoder};
use tproto::streaming::{Body, Message};
use tproto::streaming::pipeline::Frame;
use bytes::{Bytes, BytesMut};

use std::io;

pub struct AMQPCodec;

// pub struct FrameStream {
//     inner: Body<String, std::io::Error>
// }

impl Decoder for AMQPCodec {
    type Item = Frame<Bytes, Bytes, io::Error>;
    type Error = io::Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    }
}

impl Encoder for AMQPCodec {
    type Item = Frame<Bytes, Bytes, io::Error>;
    type Error = io::Error;

    fn encode(&mut self, message: Self::Item, buffer: &mut BytesMut) -> io::Result<()> {

    }
}
