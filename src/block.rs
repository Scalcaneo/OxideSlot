/*
0	4 bytes	u32	block_id	           -    Unique identifier for the block.
4	4 bytes	u32	checksum               -    Detect disk corruptions.
8	2 bytes	u16	slot_count             -    Number of rows inserted.
10	2 bytes	u16	free_space_pointer     -    Exact byte where the free space starts (Grows from the bottom).
12	12 bytes [u8; 12]	Padding / LSN  -    Reserved space to complete 24 bytes (Log sequence number) */

pub const BLOCK_SIZE: usize = 4096;
const HEADER_SIZE: usize = 24;
const SLOT_SIZE: usize = 4;

const OFFSET_BLOCK_ID: usize = 0;
#[allow(dead_code)]
const OFFSET_CHECKSUM: usize = 4;
const OFFSET_SLOT_COUNT: usize = 8;
const OFFSET_FREE_SPACE: usize = 10;

pub type SlotID = u16;

//First 24 are destinated for the header after that we have the slots (size and offset) and from the end we have the actual data
pub struct Block{
    pub raw_data: [u8; BLOCK_SIZE],
}

impl Block{
    // Create an empty block with 0's
    pub fn new(block_id: u32) -> Self {
        let mut block = Block {
            raw_data: [0u8; BLOCK_SIZE],
        };
        // Set the slot count as zero
        block.set_slot_count(0);
        // Set the space pointer to the end of the block (that is read from back to start)
        block.set_free_space_pointer(BLOCK_SIZE as u16);
        // This is the rust way to do the byte manipulation, we convert u32 to a 4 bytes array in Little Endian and we copy
        block.raw_data[OFFSET_BLOCK_ID..OFFSET_BLOCK_ID + 4].copy_from_slice(&block_id.to_le_bytes());

        block
    }

    fn get_slot_count(&self) -> u16{
        // Little Endian convertion from 8 bites to 16 bites
        (self.raw_data[OFFSET_SLOT_COUNT] as u16) | ((self.raw_data[OFFSET_SLOT_COUNT + 1] as u16) << 8)
    }

    fn get_free_space_pointer(&self) -> u16{
        (self.raw_data[OFFSET_FREE_SPACE] as u16) | ((self.raw_data[OFFSET_FREE_SPACE + 1] as u16) << 8)
    }

    fn set_slot_count(&mut self, count: u16){
        self.raw_data[OFFSET_SLOT_COUNT] = (count & 0xFF) as u8;
        self.raw_data[OFFSET_SLOT_COUNT + 1] = ((count >> 8) & 0xFF) as u8;
    }

    fn set_free_space_pointer(&mut self, pointer: u16){
        self.raw_data[OFFSET_FREE_SPACE] = (pointer & 0xFF) as u8;
        self.raw_data[OFFSET_FREE_SPACE + 1] = ((pointer >> 8) & 0xFF) as u8; 
    }

    fn get_free_space_remaining(&self) -> usize {
        let free_space_ptr = self.get_free_space_pointer() as usize;
        let slot_count = self.get_slot_count() as usize;
        let end_of_slots = HEADER_SIZE + (slot_count * SLOT_SIZE);

        free_space_ptr - end_of_slots
    }

    // Writes the data to the end of the block and a "slot pointer" (offset + length) to the beginning of the block (after header).
    pub fn insert_row(&mut self, payload: &[u8]) -> Option<SlotID> {
        let current_slots = self.get_slot_count();
        let payload_size = payload.len();

        if self.get_free_space_remaining() < (payload_size + SLOT_SIZE){
            return None; // Block full, Disk manager handle the case to ask for more blocks (blocks)
        }

        // Calculate new pointer of free space (downwards)
        let old_free_space = self.get_free_space_pointer();
        let new_free_space = old_free_space - (payload_size as u16);

        let start_idx = new_free_space as usize;
        let end_idx = start_idx + payload_size;

        self.raw_data[start_idx..end_idx].copy_from_slice(payload);

        let slot_offset = HEADER_SIZE + ((current_slots as usize) * SLOT_SIZE);
        self.raw_data[slot_offset..slot_offset + 2].copy_from_slice(&new_free_space.to_le_bytes());
        self.raw_data[slot_offset + 2..slot_offset + 4].copy_from_slice(&(payload_size as u16).to_le_bytes());

        // Update the header             
        self.set_free_space_pointer(new_free_space);
        self.set_slot_count(current_slots + 1);

        Some(current_slots)
    }

    pub fn get_row(&self, slot_id: SlotID) -> Option<&[u8]>{
        if slot_id >= self.get_slot_count(){
            return None;
        }

        let slot_index = HEADER_SIZE + ((slot_id as usize) * SLOT_SIZE);

        let offset_slice = &self.raw_data[slot_index .. slot_index + 2];
        let offset_bytes: [u8; 2] = offset_slice.try_into().unwrap();
        let absolute_offset = u16::from_le_bytes(offset_bytes) as usize;

        let size_slice = &self.raw_data[slot_index + 2 .. slot_index + SLOT_SIZE];
        let size_bytes: [u8; 2] = size_slice.try_into().unwrap();
        let payload_size = u16::from_le_bytes(size_bytes) as usize;

        let start_idx = absolute_offset;
        let end_idx = start_idx + payload_size;

        Some(&self.raw_data[start_idx..end_idx])
    }
}