use crate::io::{self, BorrowedCursor, ErrorKind, IoSlice, IoSliceMut};

pub struct Stdin;
pub struct Stdout;
pub type Stderr = Stdout;

impl Stdin {
    pub const fn new() -> Self {
        Self
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        while read < buffer.len() {
            let key = unsafe { syscall0(6) } as u8;
            if key == 0 {
                break;
            }
            buffer[read] = key;
            read += 1;
            if key == b'\n' {
                break;
            }
        }
        Ok(read)
    }

    fn read_buf(&mut self, mut cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        let mut count = 0;
        while count < cursor.capacity() {
            let key = unsafe { syscall0(6) } as u8;
            if key == 0 {
                break;
            }
            unsafe {
                cursor.as_mut().as_mut_ptr().add(count).write(key);
            }
            count += 1;
            if key == b'\n' {
                break;
            }
        }
        unsafe { cursor.advance_unchecked(count) };
        Ok(())
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        for buffer in bufs {
            let count = self.read(buffer)?;
            if count != 0 {
                return Ok(count);
            }
        }
        Ok(0)
    }

    fn is_read_vectored(&self) -> bool {
        true
    }
}

impl Stdout {
    pub const fn new() -> Self {
        Self
    }
}

impl io::Write for Stdout {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        // The console syscall accepts at most 256 bytes.
        let bytes = &bytes[..bytes.len().min(256)];
        let written = unsafe { syscall2(1, bytes.as_ptr(), bytes.len()) };
        if written == usize::MAX {
            Err(io::Error::from(ErrorKind::Other))
        } else {
            Ok(written)
        }
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        for buffer in bufs {
            if !buffer.is_empty() {
                return self.write(buffer);
            }
        }
        Ok(0)
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 1;

pub fn is_ebadf(error: &io::Error) -> bool {
    error.kind() == ErrorKind::BrokenPipe
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stdout::new())
}

unsafe fn syscall0(number: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            clobber_abi("sysv64"),
        );
    }
    result
}

unsafe fn syscall2(number: usize, pointer: *const u8, length: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            in("rdi") pointer,
            in("rsi") length,
            clobber_abi("sysv64"),
        );
    }
    result
}
