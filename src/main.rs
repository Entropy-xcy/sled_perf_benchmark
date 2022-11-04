use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};

fn sled_persistent_benchmark(size: u64) -> Result<(Duration, Duration), Box<dyn Error>> {
    let tree = sled::open("/tmp/sled_bench")?;

    let write_now = Instant::now();
    for i in 0..size {
        let key_bytes = &i.to_be_bytes();
        let value_bytes = &(i + 1).to_be_bytes();
        tree.insert(key_bytes, value_bytes)?;
    }
    tree.insert(&size.to_be_bytes(), &0_u64.to_be_bytes())?;
    let write_elapsed = write_now.elapsed();

    let read_now = Instant::now();
    // pointer tracing
    let mut pointer: u64 = 1;
    while pointer != 0 {
        let bytes = &tree.get(&pointer.to_be_bytes()).unwrap().unwrap() as &[u8];
        let bytes = bytes[0..8].try_into().unwrap();
        pointer = u64::from_be_bytes(bytes);
    }
    let read_elapsed = read_now.elapsed();

    Ok((write_elapsed, read_elapsed))
}


fn hashmap_benchmark(size: u64) -> Result<(Duration, Duration), Box<dyn Error>> {
    let write_now = Instant::now();
    let mut map = HashMap::new();
    for i in 0..size {
        map.insert(i, i + 1);
    }
    map.insert(size, 0);
    let write_elapsed = write_now.elapsed();

    let read_now = Instant::now();
    // pointer tracing
    let mut pointer = 1;
    while pointer != 0 {
        pointer = *map.get(&pointer).unwrap();
    }
    let read_elapsed = read_now.elapsed();
    Ok((write_elapsed, read_elapsed))
}

fn main() {
    let size = 1_000_000;
    let (hash_write_elapsed, hash_read_elapsed) = hashmap_benchmark(size).unwrap();
    let (sled_write, sled_read) = sled_persistent_benchmark(size).unwrap();
    println!("hashmap read elapsed: {:?}", hash_read_elapsed);
    println!("hashmap write elapsed: {:?}", hash_write_elapsed);
    println!("sled read elapsed: {:?}", sled_read);
    println!("sled write elapsed: {:?}", sled_write);
    println!("hashmap read speedup: {:?}", sled_read.as_nanos() as f64 / hash_read_elapsed.as_nanos() as f64);
    println!("hashmap write speedup: {:?}", sled_write.as_nanos() as f64/ hash_write_elapsed.as_nanos() as f64);
}
