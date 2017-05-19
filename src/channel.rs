use tcore::reactor::Core;

///
/// Channel structure of AMQP queue.
///
pub struct Channel {
    id: u16, core: Core
}
