//use csv::Error;
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[derive(Debug)]

struct Motivation{
    number: u64,
    author: String,
    phrase: String,
    spanish: String,
}

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {

    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result.String;
        let mut lines = record.lines;
        println!("{}", lines);
        //println!("{:?}", record);
        break;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "./assets/phrases.csv";
    read_csv(filename)
}

