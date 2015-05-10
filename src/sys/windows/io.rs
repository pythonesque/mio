use {io, Evented, Interest, PollOpt, Selector, Token, TryRead, TryWrite};

#[derive(Debug)]
pub struct Io;

impl Io {
}

impl Evented for Io {
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

impl TryRead for Io {
    fn read_slice(&mut self, dst: &mut [u8]) -> io::Result<Option<usize>> {
        unimplemented!();
    }
}

impl TryWrite for Io {
    fn write_slice(&mut self, src: &[u8]) -> io::Result<Option<usize>> {
        unimplemented!();
    }
}
