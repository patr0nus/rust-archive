use crate::reader::entry::Entry;
use crate::reader::seek_conv::FFISeekFrom;
use libarchive_src::{archive_read_data, archive_seek_data};
use std::io;
use std::io::SeekFrom;

#[derive(Debug)]
pub struct EntryReader<'entry, 'archive: 'entry>(Entry<'entry, 'archive>);

impl<'entry, 'archive: 'entry> From<Entry<'entry, 'archive>> for EntryReader<'entry, 'archive> {
    fn from(value: Entry<'entry, 'archive>) -> Self {
        EntryReader(value)
    }
}

impl<'entry, 'archive: 'entry> EntryReader<'entry, 'archive> {
    pub fn entry(&self) -> &Entry<'entry, 'archive> {
        &self.0
    }
}

impl<'entry, 'archive: 'entry> io::Read for EntryReader<'entry, 'archive> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            let ret = archive_read_data(self.0.archive.ptr, buf.as_mut_ptr().cast(), buf.len());
            let Ok(n) = usize::try_from(ret) else {
                return Err(self.0.archive.get_error())
            };
            Ok(n)
        }
    }
}

impl<'entry, 'archive: 'entry> io::Seek for EntryReader<'entry, 'archive> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let FFISeekFrom { offset, whence } = pos.into();
        unsafe {
            let ret = archive_seek_data(self.0.archive.ptr, offset, whence);
            let Ok(n) = u64::try_from(ret) else {
                return Err(self.0.archive.get_error())
            };
            Ok(n)
        }
    }
}
