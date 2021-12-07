use std::error::Error;
use csv::Writer;
use text_io::read;
use rand::Rng;
use std::fs::OpenOptions;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::ErrorKind;
use csv::StringRecord;



fn main() {
    let n : i32 = read!();
    let _ = generate_data_for_csv(n);
}

fn generate_data_for_csv(ntimes: i32) -> Result<(), Box<dyn Error>> {
    let _ = verify_if_file_exists();
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .open("data_generated.csv")
        .unwrap();
    let mut wtr = Writer::from_writer(&file);
    let mut vector_of_random_policies: Vec<u8> = Vec::new();
    let mut n = ntimes;
    while n > 0 {
        vector_of_random_policies.push(rand::thread_rng().gen_range(1..4));
        n -= 1;
    }

    let mut hash = DefaultHasher::new();
    vector_of_random_policies.hash(&mut hash);
    
    if let Ok(()) = checking_pre_existing_hashs(hash.finish(), &vector_of_random_policies, &file) {

        wtr.serialize((
            format!("{:?}", hash.finish()),
            format!("{:?}", vector_of_random_policies)
        ))?;
        wtr.flush()?;
    }
    Ok(())
}


fn verify_if_file_exists() -> Result<(), Box<dyn Error>> {
    let file = File::open("data_generated.csv");
    let _file = match file {
        Ok(_file) => Ok(()),
        Err(err) => match err.kind() {
                ErrorKind::NotFound => create_file(),
                _ => panic!("boom")
            },
    };
    Ok(())
}

fn create_file() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data_generated.csv")
        .unwrap();
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["hash_list", "policies"])?;
    wtr.flush()?;
    return Ok(());
}

fn checking_pre_existing_hashs(_hash: u64, _polices: &Vec<u8>, _file: &File) -> Result<(), Box<dyn Error>> {
    
    let string_polices = format!("{:?}", &_polices);
    let search_record = StringRecord::from(vec![_hash.to_string(), string_polices]);
    println!("{:?}", search_record);

    let mut rdr = csv::Reader::from_reader(_file);
    for result in rdr.records() {
        let record = result?;
        if record == search_record {
            panic!("Already existing hash");
        }
    }
    Ok(())
}