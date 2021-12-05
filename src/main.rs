use std::error::Error;
use csv::Writer;
use text_io::read;
use rand::Rng;
use std::fs::OpenOptions;

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
    let mut vector_of_random_policies: Vec<i32> = Vec::new();
    let mut n = ntimes;
    while n > 0 {
        vector_of_random_policies.push(rand::thread_rng().gen_range(1..3));
        n -= 1;
    }
    let string_vector = format!("{:?}", vector_of_random_policies);
    wtr.serialize(string_vector)?;
    wtr.flush()?;
    Ok(())
}
