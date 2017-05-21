///
/// The connection class provides methods for a client
/// to establish a network connection to a server,
/// and for both peers to operate the connection thereafter.
///
/// Grammar :
///
/// connection = open-connection *use-connection close-connection
/// open-connection = C:protocol-header S:START C:START-OK *challenge S:TUNE C:TUNE-OK C:OPEN S:OPEN-OK
/// challenge = S:SECURE C:SECURE-OK
/// use-connection = *channel
/// close-connection = C:CLOSE S:CLOSE-OK / S:CLOSE C:CLOSE-OK
///
pub trait Connection {
    use method::conn::Response::*;

    ///
    /// This method starts the connection negotiation process by telling the client the
    /// protocol version that the server proposes, along with a list of security mechanisms
    /// which the client can use for authentication.
    ///
    /// index: 10
    ///
    /// version are built in and hardcoded.
    ///
    /// The server MUST provide a protocol version that is lower than or equal to
    /// that requested by the client in the protocol header.
    ///
    /// If the client cannot handle the protocol version suggested by the server
    /// it MUST close the socket connection without sending any further data.
    ///
    fn start(&self, );

    ///
    ///
    ///
    fn secure();

    ///
    ///
    ///
    fn tune();

    ///
    ///
    ///
    fn open();

    ///
    ///
    ///
    fn close();

    ///
    ///
    ///
    fn blocked();

    ///
    ///
    ///
    fn unblocked();
}

///
/// The channel class provides methods for a client to establish a channel
/// to a server and for both peers to operate the channel thereafter.
///
/// Grammar:
///
/// channel = open-channel *use-channel close-channel
/// open-channel = C:OPEN S:OPEN-OK
/// use-channel = C:FLOW S:FLOW-OK / S:FLOW C:FLOW-OK / functional-class
/// close-channel = C:CLOSE S:CLOSE-OK / S:CLOSE C:CLOSE-OK
///
pub trait Channel {
    use method::exchange::Response::*;

    type S;

    ///
    /// index: 10
    ///
    /// - state, on-failure: channel-error
    /// The client MUST NOT use this method on an already-opened channel.
    ///
    fn open(&self);

    ///
    /// index: 20
    ///
    /// This method asks the peer to pause or restart the flow of content data sent by
    /// a consumer. This is a simple flow-control mechanism that a peer can use to avoid
    /// overflowing its queues or otherwise finding itself receiving more messages than
    /// it can process. Note that this method is not intended for window control. It does
    /// not affect contents returned by Basic.Get-Ok methods.
    ///
    fn flow(&self);

    ///
    /// index: 40
    ///
    /// This method indicates that the sender wants to close the channel. This may be due to
    /// internal conditions (e.g. a forced shut-down) or due to an error handling a specific
    /// method, i.e. an exception. When a close is due to an exception, the sender provides
    /// the class and method id of the method which caused the exception.
    ///
    ///
    fn close(&self, reply_code: S, reply_text: S, class_id: S, method_id: S);
}

///
/// Exchanges match and distribute messages across queues.
/// Exchanges can be configured in the server or declared at runtime.
///
/// Grammar :
///
/// exchange declare = C:DECLARE S:DECLARE-OK
/// exchange delete  = C:DELETE S:DELETE-OK
/// exchange bind    = C:BIND S:BIND-OK
/// exchange unbind  = C:UNBIND S:UNBIND-OK
///
/// types: fanout, direct, topic, headers
///
/// default exchange rule:
///
/// Client checks that the default exchange is active by specifying a queue
/// binding with no exchange name, and publishing a message with a suitable
/// routing key but without specifying the exchange name, then ensuring that
/// the message arrives in the queue correctly.
///
pub trait Exchange {
    use method::exchange::Response::*;

    type S;

    ///
    /// index: 10
    ///
    /// field: exchange, label: exchange-name, assert: notnull
    ///
    /// This method creates an exchange if it does not already exist, and if the exchange
    /// exists, verifies that it is of the correct and expected class.
    ///
    /// reserved exchange on-failure: access-refused
    ///
    /// Exchange names starting with "amq." are reserved for pre-declared and
    /// standardised exchanges. The client MAY declare an exchange starting with
    /// "amq." if the passive option is set, or the exchange already exists.
    ///
    /// syntax on-failure: precondition-failed
    ///
    /// The exchange name consists of a non-empty sequence of these characters:
    /// letters, digits, hyphen, underscore, period, or colon.
    ///
    /// field: type, label: exchange type
    ///
    /// typed on-failure: not-allowed
    ///
    /// Exchanges cannot be redeclared with different types.  The client MUST not
    /// attempt to redeclare an existing exchange with a different type than used
    /// in the original Exchange.Declare method.
    ///
    /// support on-failure: command-invalid
    ///
    /// The client MUST NOT attempt to declare an exchange with a type that the
    /// server does not support.
    ///
    /// field: passive, label: do not create exchange
    ///
    /// not-found
    ///
    /// If set, and the exchange does not already exist, the server MUST
    /// raise a channel exception with reply code 404 (not found).
    ///
    /// equivalent
    ///
    /// If not set and the exchange exists, the server MUST check that the
    /// existing exchange has the same values for type, durable, and arguments
    /// fields.  The server MUST respond with Declare-Ok if the requested
    /// exchange matches these fields, and MUST raise a channel exception if
    /// not.
    ///
    fn declare(&mut self, exchange: S, kind: S,
               passive: bool, durable: bool,
               auto_delete: bool, internal: bool,
               nowait: bool, arguments: table::Table) -> Result<DeclareOk>;

    ///
    /// This method deletes an exchange. When an exchange is deleted all queue bindings on
    /// the exchange are cancelled.
    ///
    /// field: exchange, assert: notnull
    ///
    /// exists, on-failure: not-found
    ///
    /// field: unused (if-unused), label: delete only if unused
    ///
    /// If set, the server will only delete the exchange if it has no queue bindings. If
    /// the exchange has queue bindings the server does not delete it but raises a
    /// channel exception instead.
    ///
    /// in-use, on-failure: precondition-failed
    ///
    /// The server MUST NOT delete an exchange that has bindings on it, if the if-unused
    /// field is true.
    ///
    fn delete(&mut self, exchange: S, unused: bool, nowait: bool) -> Result<DeleteOk>;

    ///
    /// This method binds an exchange to an exchange.
    ///
    /// field: dest (destination), label: name of the distination exchange to bind to
    ///
    /// exchange-existence, on-failure: not-found
    ///
    /// A client MUST NOT be allowed to bind a non-existent
    /// destination exchange.
    ///
    /// field: source, label: name of the source exchange to bind to
    ///
    /// exchange-existence, on-failure: not-found
    ///
    /// A client MUST NOT be allowed to bind a non-existent source
    /// exchange.
    ///
    fn bind(&mut self, dest: S, source: S, route: S,
            nowait: bool, arguments: table::Table) -> Result<BindOk>;

    ///
    /// This method unbinds an exchange from an exchange.
    ///
    /// If a unbind fails, the server MUST raise a connection exception.
    ///
    /// field: destination
    ///
    /// must-exist, on-failure: not-found
    ///
    /// The client MUST NOT attempt to unbind an exchange that
    /// does not exist from an exchange.
    ///
    /// field: source
    ///
    /// must-exist, on-failure: not-found
    ///
    fn unbind(&mut self, dest: S, source: S, route: S,
              nowait: bool, arguments: table::Table) -> Result<UnbindOk>;


}

///
/// Queues store and forward messages. Queues can be configured in the server or created at
/// runtime. Queues must be attached to at least one exchange in order to receive messages
/// from publishers.
///
/// Note about queue-name:
///
/// length max : 127
/// regexp : "^[a-zA-Z0-9-_.:]*$"
///
/// Grammar:
///
/// declare = C:DECLARE S:DECLARE-OK
/// bind    = C:BIND S:BIND-OK
/// unbind  = C:UNBIND S:UNBIND-OK
/// purge   = C:PURGE S:PURGE-OK
/// delete  = C:DELETE S:DELETE-OK
///
pub trait Queue {
    use method::queue::Response::*;

    type S;

    ///
    /// This method creates or checks a queue. When creating a new queue the client can
    /// specify various properties that control the durability of the queue and its
    /// contents, and the level of sharing for the queue.
    ///
    fn declare(&mut self, queue: S, passive: bool, durable: bool,
               exclusive: bool, auto_delete: bool, nowait: bool,
               arguments: table::Table) -> Result<DeclareOk>;

    ///
    ///
    ///
    fn bind(&mut self, queue: S, exchange: S, route: S, nowait: bool,
            arguments: table::Table) -> Result<BindOk>;

    ///
    ///
    ///
    fn unbind(&mut self, queue: S, exchange: S, route: S, nowait: bool,
              arguments: table::Table) -> Result<UnbindOk>;

    ///
    ///
    ///
    fn purge(&mut self, queue: S, nowait: bool) -> Result<PurgeOk>;

    ///
    ///
    ///
    fn delete(&mut self, queue: S, unused: bool, empty: bool, nowait: bool) -> Result<DeleteOk>;
}

///
/// Delivery mode
///
pub enum DeliveryMode {
    Persistent,
    Transient
}

pub struct BasicProperties {
    content_type: String,
    content_encoding: String,
    headers: table::Table,
    delivery_mode: DeliveryMode,
    priority: u8,
    correlation: String,
    reply_to: String,
    expiration: String,
    message_id: String,
    timestamp: ,
    kind: String,
    user_id: String,
    _reserved: String
}

///
///
///
pub trait Basic {

    use method::basic::Response::*;

    type S;

    ///
    ///
    ///
    fn qos(&mut self, prefetch_size: u64, prefetch_count: u8, global: bool) -> Result<QosOk>;

    ///
    ///
    ///
    fn consume(&mut self, queue: S, consumer_tag: S,
               nolocal: bool, noack: bool, exclusive: bool, nowait: bool,
               arguments: table::Table) -> Result<ConsumeOk>;

    ///
    ///
    ///
    fn cancel(&mut self, consumer_tag: S, nowait: bool) -> Result<CancelOk>;

    ///
    ///
    ///
    fn publish(&mut self, exchange: S, route: S, mandatory: bool, immediate: bool) -> Result<()>;

    ///
    ///
    ///
    fn return_fail(&mut self, reply_code: S, reply_text: S, exchange: S, route: S) -> Result<()>;

    ///
    ///
    ///
    fn deliver(&mut self, consumer_tag: S, delivery_tag: S, redelivered: bool, exchange: S) -> Result<()>;

    ///
    ///
    ///
    fn get(&mut self, queue: S, noack: bool) -> Result<GetOk>;

    ///
    ///
    ///
    fn ack(&mut self, delivery_tag: S, multiple: bool) -> Result<>
}

///
/// The Tx class allows publish and ack operations to be
/// batched into atomic units of work. The intention is
/// that all publish and ack requests issued within a
/// transaction will complete successfully or none of them will.
/// Servers SHOULD implement atomic transactions at least where
/// all publish or ack requests affect a single queue.
/// Transactions that cover multiple queues may be non-atomic,
/// given that queues can be created and destroyed asynchronously,
/// and such events do not form part of any transaction.
/// Further, the behaviour of transactions with respect to the
/// immediate and mandatory flags on Basic.Publish methods is not defined.
///
/// Rules:
///
/// - not multiple queues
/// Applications MUST NOT rely on the atomicity of transactions that
/// affect more than one queue.
///
/// - not immediate
/// Applications MUST NOT rely on the behaviour of transactions that
/// include messages published with the immediate option.
///
/// - not mandatory
/// Applications MUST NOT rely on the behaviour of transactions that
/// include messages published with the mandatory option.
///
pub trait Transaction {

    use method::trans::Response::*;

    ///
    ///
    ///
    ///
    fn select(&mut self) -> Result<SelectOk>;

    ///
    ///
    ///
    fn commit(&mut self) -> Result<CommitOk>;

    ///
    ///
    ///
    ///
    fn rollback(&mut self) -> Result<RollbackOk>;
}


///
/// The Confirm class allows publishers to put the channel in confirm mode
/// and subsequently be notified when messages have been handled by the broker.
/// The intention is that all messages published on a channel in confirm mode
/// will be acknowledged at some point. By acknowledging a message
/// the broker assumes responsibility for it and indicates that
/// it has done something it deems reasonable with it.
/// Unroutable mandatory or immediate messages are acknowledged right
/// after the Basic.Return method. Messages are acknowledged when all queues
/// to which the message has been routed have either delivered the message
/// and received an acknowledgement (if required), or enqueued the message
/// (and persisted it if required). Published messages are assigned
/// ascending sequence numbers, starting at 1 with the first Confirm.Select method.
/// The server confirms messages by sending Basic.Ack methods referring to these sequence numbers.
///
/// Rules:
///
/// - all messages acknowledged
/// The server MUST acknowledge all messages received after the channel was put into confirm mode.
///
/// - all queues
/// The server MUST acknowledge a message only after it was properly handled by all the queues
/// it was delivered to.
///
/// - unroutable messages
/// The server MUST acknowledge an unroutable mandatory or immediate message
/// only after it sends the Basic.Return.
///
/// - time guarantees
/// No guarantees are made as to how soon a message is acknowledged.
/// Applications SHOULD NOT make assumptions about this.
///
pub trait Confirm {

    use method::confim::Response::*;

    ///
    ///
    ///
    fn select(&mut self, nowait: bool) -> Result<SelectOk>;
}
