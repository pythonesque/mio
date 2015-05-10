use {io, Evented, Interest, PollOpt, Selector, Token};

#[derive(Debug)]
pub struct Awakener;

impl Awakener {
    pub fn new() -> io::Result<Awakener> {
        unimplemented!();
    }

    pub fn wakeup(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn cleanup(&self) {
        unimplemented!();
    }
}

impl Evented for Awakener {
    fn register(&self, selector: &mut Selector, token: Token, interest: Interest, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn reregister(&self, selector: &mut Selector, token: Token, interest: Interest, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, selector: &mut Selector) -> io::Result<()> {
        unimplemented!();
    }
}
