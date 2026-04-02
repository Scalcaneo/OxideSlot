use std::collections::HashMap;
use crate::block::{Block, BLOCK_SIZE};
use crate::disk_manager::DiskManager;
use crate::lru_replacer::LRUReplacer;

pub struct BufferPoolManager {
    // 1. The physic memory (Frames)
    // Here is where the block in RAM are. If capacity is 10, This array will be 10.
    frames: Vec<Block>,

    // 2. Logic map (disk block id -> FrameID in the RAM)
    // Tells if a block is already in memory.
    page_table: HashMap<u32, usize>,

    // 3. Frames metadata (parallel arrays to avoid complex Mutex)
    // is_dirty[i] tells if Frame 'i' was modified and if it needs to be stored before destrying it.
    is_dirty: Vec<bool>,
    // pin_count[i] How many threads are using Frame 'i' at same time.
    pin_count: Vec<usize>,
    
    // 4. Main components
    disk_manager: DiskManager,
    replacer: LRUReplacer,
    
    // 5. Limits
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
            page_table: HashMap::with_capacity(pool_size),
            is_dirty: vec![false; pool_size],
            pin_count: vec![0; pool_size],
            disk_manager,
            replacer: LRUReplacer::new(pool_size),
            pool_size,
        }
    }

    // --- FUNCTIONS TO IMPLEMENT ---

    // 1. Bring a block from disk (or memory if already loaded)
    pub fn fetch_page(&mut self, block_id: u32) -> Option<&mut Block> {
        // TODO
        None
    }

    // 2. Let the buffer pool know that we finished using the block
    pub fn unpin_page(&mut self, block_id: u32, is_dirty: bool) -> bool {
        // TODO
        false
    }
}