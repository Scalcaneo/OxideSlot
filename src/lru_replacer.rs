use std::{collections::HashMap, ptr::null};

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
    fn remove_node(&mut self, index: usize) {
        let node = self.nodes[index];

        match node.prev{
            // Handling the case where we have a prev node
            Some(prev_idx) => self.nodes[prev_idx].next = node.next,
            // Handling the case where we dont have a prev
            None => self.head = node.next,
        }

        match node.next{
            Some(next_idx) => self.nodes[next_idx].prev = node.prev,
            None => self.tail = node.prev,
        }
        self.nodes[index].prev = None;
        self.nodes[index].next = None;

    }
    
    // Insert a node in the head (MRU)
    fn push_front(&mut self, index: usize) {
        self.nodes[index].prev = None;
        self.nodes[index].next = self.head;

        match self.head {
            Some(head) => self.nodes[head].prev = Some(index),
            None => self.tail = Some(index),
        }
        self.head = Some(index);
    }
}