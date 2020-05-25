use std::error::Error;
use std::io::prelude::*;
use rand::Rng;
use std::fs;

// parse cli args into struct
pub struct Parse {
    pub filename: String,
    pub rounds: u32,
}

impl Parse {
    pub fn new(mut args: std::env::Args) -> Result<Parse, &'static str> {
        args.next();
        
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename provided"),
        };

        let rounds :u32 = match args.next() {
            Some(arg) => arg.parse::<u32>().unwrap(),
            None => return Err("provided the number of rounds"),
        };
        
        Ok(Parse {filename, rounds})
    }
}

pub fn shred(parse: Parse) -> Result<(), Box<dyn Error>> {
    let mut filename = parse.filename;
    let file_size = fs::metadata(&filename).unwrap().len() as usize;

    println!("Shred It");

    for _round in 1..=parse.rounds {
        fill(&filename, "zero", file_size);
        fill(&filename, "random", file_size);
        filename = rand_name(&filename);
        }
    
    fs::remove_file(&filename).expect("File remove failed");

    Ok(())
}

fn fill(filename: & String, content: &str, size: usize) {
    let mut rng = rand::thread_rng();
    let mut filler = vec![0; size];
    
    if content == "random" {
        filler = (0..size).map(|_| rng.gen()).collect();
    }

    let mut f = fs::File::create(&filename)
        .expect("Failed to access file");

    f.write_all(&filler)
        .expect("Failed to write to file");

    f.sync_all()
        .expect("Failed to sync file");
}

fn rand_name(filename: & String) -> String {
    let mut rng = rand::thread_rng();
    let r_name = rng.gen::<[u8; 16]>();
    let name = base64::encode_config(r_name, base64::URL_SAFE_NO_PAD);
   
    let new_name = name;
    fs::rename(filename, &new_name)
        .expect("Failed to rename file");
    
    return new_name;
}

