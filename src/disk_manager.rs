use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use crate::block::BLOCK_SIZE;

pub struct DiskManager {
    file: File,
}

    impl DiskManager{
        pub fn new(file_path: &str) -> Self{
            //OpenOptions to open the file, read,write and create it if it does not exist
            let file= OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(file_path)
                .expect("FATAL: Unable to open or create .db file");

            Self{
                file
            }
        }

        pub fn write_block(&mut self, block_id: u32, block_data: &[u8]){
            // Calculate the correct offset
            let offset: usize = block_id as usize * BLOCK_SIZE;
            // Put the pointer of the file to the offset value
            self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
            // Write to the block(block) the input bytes
            self.file.write_all(block_data).unwrap();
        }

        pub fn read_block(&mut self, block_id: u32) -> [u8; BLOCK_SIZE]{
            let offset: usize = block_id as usize * BLOCK_SIZE;
            self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
            let mut buffer = [0u8; BLOCK_SIZE];
            
            self.file.read_exact(&mut buffer).unwrap();

            buffer
        }
    
    }