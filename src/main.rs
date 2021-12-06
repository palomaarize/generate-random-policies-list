use std::error::Error;
use csv::Writer;
use csv::Reader;
use csv::StringRecord;
use std::io::BufReader;
use csv::ReaderBuilder;
use text_io::read;
use rand::Rng;
use std::fs::OpenOptions;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::collections::HashMap;
use serde::Deserialize;



fn main() {
    let n : i32 = read!();
    let _ = generate_data_csv(n);
    //let teste = example();
}

fn generate_data_csv(ntimes: i32) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
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
    println!("{}", hash.finish().to_string());
    
    let checked = checking_pre_existing_hashs(hash.finish(), &vector_of_random_policies, &file);
    println!("{:?}", checked);
    if let Ok(()) = checked {

        wtr.serialize((
            format!("{:?}", hash.finish()),
            format!("{:?}", vector_of_random_policies)
        ))?;
        wtr.flush()?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Row {
    hash: String,
    police: Vec<u8>
}

fn checking_pre_existing_hashs(_hash: u64, _polices: &Vec<u8>, _file: &File) -> Result<(), Box<dyn Error>> {

    //let string2: String = String::from_utf8(_polices.clone()).unwrap();

    let rdr = Reader::from_reader(_file);
    let mut iter = rdr.into_deserialize();

    println!("{:?}", _polices.to_vec());

    if let Some(result) = iter.next() {
        let record: Row = result?;
        assert_eq!(record, Row {
            hash: _hash.to_string(),
            police: _polices.to_vec(),
        });
        Err(From::from("expected at least one record but got none"))
    } else {
        Ok(())
    }
}