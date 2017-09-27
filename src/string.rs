use std::str;
use std::io::{Error, ErrorKind};
use std::io::Write;

pub struct Buffer (String);

impl Buffer {
    pub fn new() -> Buffer {
        Buffer(String::new())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Write for Buffer {
    fn write(&mut self, src: &[u8]) -> ::std::io::Result<usize> {
        let s = str::from_utf8(src).map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;
        self.0.push_str(&s);
        Ok(src.len())
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.0.truncate(0);
        Ok(())
    }
}

//TODO: read and advance the offset
//impl Read for Buffer {
//    fn read(&mut self, dst: &mut [u8]) -> ::std::result::Result<usize, ::std::io::Error> {
//        let l = dst.len()
//        read = copy(dst, self.src)
//        self.off += read
//        Ok(read)
//    }
//}

