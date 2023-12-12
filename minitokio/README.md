Seems I can't do what I want with mio ...

I would like to have the event loop in a dedicated thread.

To reach that I should be able to send the registration to the thread
(and receive events, but I'll keep this story for later).

But mio registry seems not to be compatible with this.

Idealy, for my simple case, I would not need to send informations
like those parameters:

```rust
pub fn register<S>(&self, source: &mut S, token: Token, interests: Interest) -> io::Result<()>
    where
        S: event::Source + ?Sized,
```

as I would only need the raw_fd of the Poll instance (yes, well, unsafe so ...)

But how to write a function with `event::Source` trait bound as it is not public ?