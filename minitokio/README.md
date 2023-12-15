# What came through my mind

## throwing away mio ?

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

## It could work without but ...

It happens that kind of code was going to work, inspired by:

https://www.zupzup.org/epoll-with-rust/index.html

```rust

pub fn run(&self) {
        let epoll_fd = self.epoll_fd;

        thread::spawn(move || {
            println!("Thread running");
            let mut event_list: Vec<libc::epoll_event> = Vec::with_capacity(1024);

            event_list.clear();

            let res = syscall!(epoll_wait(
                epoll_fd,
                event_list.as_mut_ptr() as *mut libc::epoll_event,
                1024,
                10000
            )).unwrap();


```

Turned out ... I received an EPOLLERR (server was not listening), and if I wanted to go
further I had to reimplement a lot of boilerplate code already in mio like:

```rust
pub fn is_error(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLERR) != 0
    }
```

and clearly sharing the poll fd was working, but didn't fell to Rusty

And then I remembered the pattern in the rust book to share type that are not Send and Sync:

Arc<Mutex<T>>

So I am going back to use mio and try this way

## It works better without Mutex

Noob mistake, a mutex at the start of a inite loop ... is a recipe for race conditions
Even with a small scope, it needs long operation to release the mutex.

Found an interesting solution for my needs in tokio source code with try_clone:

```rust
        // try_clone allows to creates a new independently owned Registry.
        // Event sources registered with this Registry will be registered 
        // with the original Registry and Poll instance.
        let registry = poll.registry().try_clone().unwrap();
```

So no need for a mutex and I can have write access to the registry outside the
inifinite loop on poll.