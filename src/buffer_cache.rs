use std::collections::HashMap;
use crate::block::{Block, BLOCK_SIZE};
use crate::disk_manager::DiskManager;
use crate::lru_replacer::LRUReplacer;

pub struct BufferPoolManager {
    // The physic memory (Frames)
    // Here is where the block in RAM are. If capacity is 10, This array will be 10.
    frames: Vec<Block>,

    // Logic map (disk block id -> FrameID in the RAM)
    // Tells if a block is already in memory.
    blocks_table: HashMap<u32, usize>,

    // Frames metadata (parallel arrays to avoid complex Mutex)
    // is_dirty[i] tells if Frame 'i' was modified and if it needs to be stored before destrying it.
    is_dirty: Vec<bool>,
    // pin_count[i] How many threads are using Frame 'i' at same time.
    pin_count: Vec<usize>,
    
    // Main components
    disk_manager: DiskManager,
    replacer: LRUReplacer,
    
    // Limits
    pool_size: usize,
}

impl BufferPoolManager {
    pub fn new(pool_size: usize, disk_manager: DiskManager) -> Self {
        //Rust trick: Initialize frame arrays with empty blocks
        let mut frames = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            frames.push(Block::new(0)); // dummy block
        }

        Self {
            frames,
            blocks_table: HashMap::with_capacity(pool_size),
            is_dirty: vec![false; pool_size],
            pin_count: vec![0; pool_size],
            disk_manager,
            replacer: LRUReplacer::new(pool_size),
            pool_size,
        }
    }

    // --- FUNCTIONS TO IMPLEMENT ---

    //Bring a block from disk (or memory if already loaded)
    pub fn fetch_page(&mut self, block_id: u32) -> Option<&mut Block> {
        if let Some(&frame_index) = self.blocks_table.get(&block_id){
            self.pin_count[frame_index] += 1;
            self.replacer.pin(frame_index);
            return Some(&mut self.frames[frame_index]);
        }

        let victim_frame = self.find_victim_frame()?;
        self.evict_old_block(victim_frame);

        self.frames[victim_frame].raw_data = self.disk_manager.read_block(block_id);
        self.blocks_table.insert(block_id, victim_frame);
        self.pin_count[victim_frame] = 1;
        self.is_dirty[victim_frame] = false;
        self.replacer.pin(victim_frame);

        Some(&mut self.frames[victim_frame])

    }

    //Let the buffer pool know that we finished using the block
    pub fn unpin_page(&mut self, block_id: u32, is_dirty: bool) -> bool {
        // TODO
        false
    }

        // HELPERS

    // Find physic RAM space
    fn find_victim_frame(&mut self) -> Option<usize>{
        if self.blocks_table.len() < self.pool_size{
            Some(self.blocks_table.len())
        }
        else{
            self.replacer.victim()
        }
    }

    // Save dirty data on disk and remove old block from map
    fn evict_old_block(&mut self, frame_index: usize){
        let old_block_bytes: [u8; 4] = self.frames[frame_index].raw_data[0..4].try_into().unwrap();
        let old_block_id = u32::from_le_bytes(old_block_bytes);

        if self.blocks_table.contains_key(&old_block_id){
            if self.is_dirty[frame_index]{
                self.disk_manager.write_block(old_block_id, &self.frames[frame_index].raw_data);
            }
            self.blocks_table.remove(&old_block_id);
        }
    }

    
}