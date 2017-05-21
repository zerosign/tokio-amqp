# tokio-amqp
amqp client implementation using tokio

This project are based on RabbitMQ amqp client protocols
[specs](https://www.rabbitmq.com/resources/specs/amqp0-9-1.extended.xml).
Support for amqp 1.0 will be implemented later
(this could be implemented in different crates).

I'm trying hard to implement the spec from RabbitMQ as
match as possible.

There are several checks that you can enable that respect
from actual specs. You could enable this by enabling
feature flag `runtime_check`.

By implementing amqp client in tokio, we could switch from
async and sync implementation easily by unwrapping the future
itself. Having this aside, several implementation defaults
will be async, ex: consumer section.

# TODO

[-] Implement and document the amqp protocols based on traits.
[-] Implement connection & channel pools.
[-] Implement the wire protocols using tokio-proto.
[-] Create several tests based on the specs.
