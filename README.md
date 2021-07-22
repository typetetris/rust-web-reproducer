# rust-web-reproducer
Trying to reproduce a strange actix-web behaviour

This is heavily machine dependent also, so you probably have to play around to
find the parameters which trigger the behaviour.

So `client_timeout` and `client_shutdown` are set to values, which should disable
those timeouts or make them irrelevant in this example.

But still I see connection closed and/or http status codes 408 in the example.

To use it, first run

    setup.sh

it will compile the used programs and it will use sudo to add some more
IPs to your loopback device, so we don't run out of ports.

Now use

    run.sh 100000

for example, this runs fine on my machine. But

    run.sh 200000

runs for around 10 seconds, but I see channel closed, connection closed by peer
and sometimes http status code 408, which to my understanding shouldn't be
the case.
