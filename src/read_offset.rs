use std::fs::File;
use std::io;


#[cfg(unix)]
#[inline]
pub(crate) fn read_offset(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::os::unix::fs::FileExt;
    file.read_at(buf, offset)
}


#[cfg(windows)]
#[inline]
pub(crate) fn read_offset(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::os::windows::fs::FileExt;
    file.seek_read(buf, offset)
}