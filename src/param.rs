use std::net::SocketAddr;
use std::default::Default;
use std::convert::{TryFrom, TryInto};
use url::{Url, ParseError};

#[derive(Debug, PartialEq, Clone)]
pub struct Credential {
    name: String,
    password: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Host {
    target: SocketAddr,
    vhost: String,
    secure: bool
}

// pub type Resource = (Credential, Host);

// impl TryFrom<Url> for Resource {
//     type Error = ParseError;

//     fn try_from(value: Url) -> Result<Self, Self::Error> {
//         let vhost = value.path_segments().unwrap_or_else(vec![]).first();
//         let secure = match value.scheme() {
//             "amqp" => false,
//             "amqps" => true
//         };

//         let target = value.host_str().unwrap_or("127.0.0.1");
//         let username = value.username();
//         let password = value.password().unwrap_or("guest");

//         Ok((
//             Credential {
//                 username: username, password: password
//             },
//             Host {
//                 target: target, vhost: vhost, secure: secure
//             }
//         ))
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Limit {
    pub channel: u16,
    pub frame: u32
}

impl Default for Credential {
    fn default() -> Credential {
        Credential {
            name: String::from("guest"),
            password: String::from("guest")
        }
    }
}

/// we don't support tls yet
impl Default for Host {
    fn default() -> Host {
        Host {
            target: "127.0.0.1:5673".parse().unwrap(),
            secure: false,
            vhost: String::from("/")
        }
    }
}

impl Default for Limit {
    fn default() -> Limit {
        Limit {
            channel: 65535,
            frame: 131072
        }
    }
}
