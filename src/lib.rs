#![feature(try_from)]
#![feature(slice_get_slice)]
#![feature(associated_type_defaults)]

extern crate tokio_proto as tproto;
extern crate tokio_core as tcore;
extern crate tokio_io as tio;
extern crate tokio_service as tserv;
extern crate bytes;
extern crate url;
extern crate chrono;
extern crate futures;
// #[macro_use]
// extern crate serde;

pub mod param;
pub mod error;
pub mod table;
pub mod method;
pub mod pool;
pub mod session;
pub mod channel;
pub mod codec;
pub mod proto;
