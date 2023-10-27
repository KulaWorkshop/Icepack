use crate::archive::ArchiveType;
use crate::lzrw;

use byteorder::{LittleEndian, WriteBytesExt};
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

use flate2::bufread::ZlibEncoder;

pub struct Builder {
    ty: ArchiveType,
    entries: Vec<FileEntry>,
}

struct FileEntry {
    name: Option<String>,
    buffer: Vec<u8>,
}

impl Builder {
    pub fn new(ty: ArchiveType) -> Builder {
        Builder {
            ty,
            entries: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: &Path) -> Result<(), Error> {
        let mut file = File::open(path)?;

        let entry: FileEntry = match self.ty {
            ArchiveType::Pak => {
                let reader = BufReader::new(file);
                let mut encoder = ZlibEncoder::new(reader, Compression::default());
                let mut buffer = Vec::new();
                encoder.read_to_end(&mut buffer)?;

                let name = path.file_name().unwrap();

                FileEntry {
                    name: Some(String::from(name.to_str().unwrap())),
                    buffer,
                }
            }
            ArchiveType::Kub => {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                FileEntry {
                    name: None,
                    buffer: lzrw::compress_buffer(buffer.as_ptr(), buffer.len()),
                }
            }
        };

        self.entries.push(entry);
        Ok(())
    }

    fn pack_kub(&self, writer: &mut BufWriter<File>) -> Result<(), Error> {
        for _ in 0..self.entries.len() {
            writer.write_u64::<LittleEndian>(0)?;
        }

        for (i, e) in self.entries.iter().enumerate() {
            let offset = writer.stream_position()? as u32;
            let size = e.buffer.len() as u32;

            writer.write_all(&e.buffer)?;
            writer.seek(SeekFrom::Start((4 + (i * 8)) as u64))?;
            writer.write_u32::<LittleEndian>(offset)?;
            writer.write_u32::<LittleEndian>(size)?;
            writer.seek(SeekFrom::End(0))?;
        }

        Ok(())
    }

    fn pack_pak(&self, writer: &mut BufWriter<File>) -> Result<(), Error> {
        for _ in 0..self.entries.len() {
            writer.write_u64::<LittleEndian>(0)?;
            writer.write_u32::<LittleEndian>(0)?;
        }

        let file_count = self.entries.len();
        for (i, e) in self.entries.iter().enumerate() {
            let name_offset = writer.stream_position()? as u32;
            let name = e.name.clone().unwrap() + "\n\0";
            writer.write_all(name.as_bytes())?;
            writer.seek(SeekFrom::Start(((4 + file_count * 8) + 4 * i) as u64))?;
            writer.write_u32::<LittleEndian>(name_offset)?;
            writer.seek(SeekFrom::End(0))?;
        }

        for (i, e) in self.entries.iter().enumerate() {
            let offset = writer.stream_position()? as u32;
            writer.write_all(&e.buffer)?;

            writer.seek(SeekFrom::Start((4 + (8 * i)) as u64))?;
            writer.write_u32::<LittleEndian>(offset)?;
            writer.write_u32::<LittleEndian>(e.buffer.len() as u32)?;
            writer.seek(SeekFrom::End(0))?;
        }

        Ok(())
    }

    pub fn pack(&self, path: &Path) -> Result<(), Error> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_u32::<LittleEndian>(self.entries.len() as u32)?;

        match self.ty {
            ArchiveType::Kub => self.pack_kub(&mut writer),
            ArchiveType::Pak => self.pack_pak(&mut writer),
        }
    }
}
