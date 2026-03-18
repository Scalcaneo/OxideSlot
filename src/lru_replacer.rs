use std::collections::HashMap;

// A node in our simulated double linked list (to avoid using strict raw pointer *mut due to borrow checker constrains)
#[derive(Clone, Copy)]
struct LRUNode {
    frame_id: usize,
    prev: Option<usize>, // index of previous node
    next: Option<usize>, //index of the next node
}

pub struct LRUReplacer {
    // Our "RAM" for the nodes
    nodes: Vec<LRUNode>,
    // O(1) Lookup: frame_id -> index in the array nodes
    frame_map: HashMap<usize, usize>,
    // Pointer to the most recent element used (MRU)
    head: Option<usize>,
    // Pointer to the least recent used (LRU - our victim)
    tail: Option<usize>,
    // Max tracking frames capacity
    capacity: usize,
}

impl LRUReplacer {
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            frame_map: HashMap::with_capacity(capacity),
            head: None,
            tail: None,
            capacity,
        }
    }

    // --- PUBLIC METHODS FOR THE REPLACER ---
    
    // pub fn victim(&mut self) -> Option<usize> { ... }
    // pub fn unpin(&mut self, frame_id: usize) { ... }
    // pub fn pin(&mut self, frame_id: usize) { ... }

    // --- PRIVATE AUX METHODS (The magic of our linked list) ---
    
    // Extract a node from the linked list connecting it 'prev' withe the 'next'
    // fn remove_node(&mut self, index: usize) { ... }
    
    // Insert a node in the head (MRU)
    // fn push_front(&mut self, index: usize) { ... }
}