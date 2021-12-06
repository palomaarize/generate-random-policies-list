use std::error::Error;
use csv::Writer;
use text_io::read;
use rand::Rng;
use std::fs::OpenOptions;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


fn main() {
    let n : i32 = read!();
    let _ = generate_data_csv(n);
}


fn generate_data_csv(ntimes: i32) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("data_generated.csv")
        .unwrap();
    let mut wtr = Writer::from_writer(file);
    let mut vector_of_random_policies: Vec<u8> = Vec::new();
    let mut n = ntimes;
    while n > 0 {
        vector_of_random_policies.push(rand::thread_rng().gen_range(1..4));
        n -= 1;
    }
    let mut hash = DefaultHasher::new();
    vector_of_random_policies.hash(&mut hash);
    wtr.serialize((
        format!("{:?}", hash.finish()),
        format!("{:?}", vector_of_random_policies)
    ))?;
    wtr.flush()?;
    Ok(())
}
