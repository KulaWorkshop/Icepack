use crate::lzrw;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;

pub enum ArchiveType {
    Pak,
    Kub,
}

pub struct Archive {
    entries: Vec<FileEntry>,
    reader: BufReader<File>,
}

pub struct ArchiveIterator<'a> {
    index: u32,
    len: u32,
    entries: &'a Vec<FileEntry>,
}

pub struct FileEntry {
    pub name: Option<String>,
    pub offset: u32,
    pub size: u32,
    buffer: Vec<u8>,
    ty: ArchiveType,
}

impl Archive {
    pub fn open(reader: BufReader<File>) -> Archive {
        Archive {
            reader,
            entries: Vec::new(),
        }
    }

    pub fn entries(&mut self) -> Result<ArchiveIterator, Error> {
        let iterator = match self.get_type()? {
            ArchiveType::Pak => self.get_pak_entries()?,
            ArchiveType::Kub => self.get_kub_entries()?,
        };

        Ok(iterator)
    }

    fn get_type(&mut self) -> Result<ArchiveType, std::io::Error> {
        self.reader.seek(SeekFrom::Start(4))?;

        let offset = self.reader.read_u32::<LittleEndian>()?;
        self.reader.seek(SeekFrom::Start(offset as u64))?;

        let check = self.reader.read_u16::<LittleEndian>()?;
        Ok(match check {
            0x9C78 => ArchiveType::Pak,
            _ => ArchiveType::Kub,
        })
    }

    fn get_kub_entries(&mut self) -> Result<ArchiveIterator, std::io::Error> {
        self.reader.seek(SeekFrom::Start(0))?;

        let file_count = self.reader.read_u32::<LittleEndian>()?;

        for i in 0..file_count {
            let offset: u32 = self.reader.read_u32::<LittleEndian>()?;
            let size: u32 = self.reader.read_u32::<LittleEndian>()?;

            let mut buffer = vec![0u8; size as usize];
            self.reader.seek(SeekFrom::Start(offset as u64))?;
            self.reader.read_exact(&mut buffer)?;
            self.reader
                .seek(SeekFrom::Start((4 + (i + 1) * 8) as u64))?;

            self.entries.push(FileEntry {
                name: None,
                offset,
                size,
                buffer,
                ty: ArchiveType::Kub,
            });
        }

        Ok(ArchiveIterator {
            len: file_count,
            index: 0,
            entries: &self.entries,
        })
    }

    fn get_pak_entries(&mut self) -> Result<ArchiveIterator, std::io::Error> {
        self.reader.seek(SeekFrom::Start(0))?;

        let file_count = self.reader.read_u32::<LittleEndian>()?;

        for i in 0..file_count {
            let offset: u32 = self.reader.read_u32::<LittleEndian>()?;
            let size: u32 = self.reader.read_u32::<LittleEndian>()?;

            let position = self.reader.stream_position()?;

            self.reader
                .seek(SeekFrom::Current((file_count * 8 - i * 4 - 8) as i64))?;
            let name_offset: u32 = self.reader.read_u32::<LittleEndian>()?;
            self.reader.seek(SeekFrom::Start(name_offset as u64))?;

            let mut name = String::new();
            self.reader.read_line(&mut name)?;
            name = name.trim_end().to_string();

            let mut buffer = vec![0u8; size as usize];
            self.reader.seek(SeekFrom::Start(offset as u64))?;
            self.reader.read_exact(&mut buffer)?;
            self.reader
                .seek(SeekFrom::Start((4 + (i + 1) * 8) as u64))?;

            self.entries.push(FileEntry {
                name: Some(name),
                offset,
                size,
                buffer,
                ty: ArchiveType::Pak,
            });

            self.reader.seek(SeekFrom::Start(position))?;
        }

        Ok(ArchiveIterator {
            len: file_count,
            index: 0,
            entries: &self.entries,
        })
    }
}

impl<'a> Iterator for ArchiveIterator<'a> {
    type Item = &'a FileEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let result = Some(&self.entries[self.index as usize]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl FileEntry {
    pub fn unpack(&self, path: &Path) -> Result<usize, std::io::Error> {
        match self.ty {
            ArchiveType::Pak => self.unpack_pak(path),
            ArchiveType::Kub => self.unpack_kub(path),
        }
    }

    fn unpack_kub(&self, path: &Path) -> Result<usize, std::io::Error> {
        let (buffer, size) = lzrw::decompress_buffer(self.buffer.as_ptr(), self.size);
        let mut file = File::create(path)?;
        file.write_all(buffer)?;
        Ok(size as usize)
    }

    fn unpack_pak(&self, path: &Path) -> Result<usize, std::io::Error> {
        let mut decoder = ZlibDecoder::new(&self.buffer[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        let mut file = File::create(path)?;
        file.write_all(&decompressed)?;
        Ok(decompressed.len())
    }
}
