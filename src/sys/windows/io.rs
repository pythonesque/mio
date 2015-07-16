use {io, Evented, EventSet, PollOpt, Selector, Token};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Io;

impl Io {
}

impl Evented for Io {
    fn register(&self, selector: &mut Selector, token: Token, interest: EventSet, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn reregister(&self, selector: &mut Selector, token: Token, interest: EventSet, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, selector: &mut Selector) -> io::Result<()> {
        unimplemented!();
    }
}

impl Read for Io {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }
}

impl Write for Io {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!();
    }
}
