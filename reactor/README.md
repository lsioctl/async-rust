# Working project to better understand async in Rust

Inspired from:

https://www.zupzup.org/epoll-with-rust/index.html

Note: reactor seems to be a bad name, I stop working
on it for now, and prefer a bit higher level with MIO
as my interest is how the Futures in Rust are coupled
with event loops (link between an "epoll" (epoll/kqueue/iocp)
and poll of Rust futures).

If I want to dig more, I have to be wary of what are the best
practices with sockets, e.g:

https://stackoverflow.com/questions/10187347/async-connect-and-disconnect-with-epoll-linux

and for storage (as it seems a thread pool dedicated to blocking instances is the
way of doing it now) (Question: and what about io_uring ?)