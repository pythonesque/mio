use {io, Evented, EventSet, PollOpt, Selector, Token};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Awakener;

impl Awakener {
    pub fn new() -> io::Result<Awakener> {
        Ok(Awakener)
    }

    pub fn wakeup(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn cleanup(&self) {
        unimplemented!();
    }
}

impl Evented for Awakener {
    fn register(&self, selector: &mut Selector, token: Token, events: EventSet, opts: PollOpt) -> io::Result<()> {
        Ok(())
        // unimplemented!();
    }

    fn reregister(&self, selector: &mut Selector, token: Token, events: EventSet, opts: PollOpt) -> io::Result<()> {
        Ok(())
        // unimplemented!();
    }

    fn deregister(&self, selector: &mut Selector) -> io::Result<()> {
        unimplemented!();
    }
}
