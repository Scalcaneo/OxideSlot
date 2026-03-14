# OxideSlot
OxideSlot is a miniature relational database storage engine built from scratch in Rust. This is strictly an educational project designed to explore the low-level mechanics of how databases store, manage, and retrieve data on disk.

The engine is currently being built from the bottom up. The foundational storage layers are complete:

### 1. Disk Manager (`disk_manager.rs`)
Handles the direct physical I/O operations. It manages a raw data file (e.g., `database.log`) and provides a simple interface to read and write fixed-size data blocks (pages) to and from the disk using precise file offsets.

### 2. Slotted Pages (`block.rs`)
Implements the standard slotted page architecture for variable-length records.
* **Block Size:** Fixed at 4KB (`4096 bytes`).
* **Header (24 bytes):** Stores metadata including `block_id`, `slot_count`, and a `free_space_pointer`.
* **Memory Layout:** The slot array (pointers and lengths) grows dynamically forwards from the end of the header, while the actual row data (payloads) is inserted backwards from the end of the block.
* **Safety:** Safely rejects payloads that exceed the available contiguous free space.

## 🚀 Usage Example

```rust
use block::Block;
use disk_manager::DiskManager;
use std::str;

fn main() {
    let mut disk = DiskManager::new("database.log");
    let mut block = Block::new(0);

    // Insert records (grows from the bottom of the 4KB page)
    block.insert_row("First record".as_bytes());
    block.insert_row("Second record".as_bytes());

    // Write the formatted page to disk
    disk.write_block(0, &block.raw_data);

    // Read it back into a new block in memory
    let mut recovered_block = Block::new(1);
    recovered_block.raw_data = disk.read_block(0);

    // Retrieve and decode the data via the slot directory
    let recovered_bytes = recovered_block.get_row(1).unwrap();
    println!("Recovered: {}", str::from_utf8(recovered_bytes).unwrap()); 
    // Output: Recovered: Second record
}

🗺️ Roadmap / Next Steps

    [x] Disk Manager: Raw file I/O and block addressing.

    [x] Slotted Pages: Byte-level tuple insertion and retrieval.

    [ ] Buffer Pool Manager: In-memory caching system to minimize disk I/O, implementing a page replacement policy.

    [ ] B+ Tree Indexing: (Optional/Future) To allow for fast exact-match and range queries.
