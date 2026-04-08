use std::str;

mod block;
mod disk_manager;
mod lru_replacer;
mod buffer_cache;

use block::Block;
use disk_manager::DiskManager;

fn main() {
    let mut block1 = Block::new(0);
    let mut disk = DiskManager::new("database.log");

    block1.insert_row("First thing inserted".as_bytes());
    block1.insert_row("Second thing inserted".as_bytes());
    block1.insert_row("Thrid thing inserted".as_bytes());

    disk.write_block(0, &block1.raw_data);
    drop(block1);
    println!("Block destroyed in RAM");

    let mut block2 = Block::new(1);
    block2.raw_data = disk.read_block(0);

    let recovered_bytes = block2.get_row(1).unwrap();
    let word = str::from_utf8(recovered_bytes).unwrap();

    println!("Recovery data from disk: {}", word);

    let stack_buffer: [u8; 5000] = [0; 5000];
    let result  = block2.insert_row(&stack_buffer);

    assert_eq!(result, None, "FATAL: Payload accepted while it is greater than the block size");
    println!("Success: The payload was safely rejected.");

}