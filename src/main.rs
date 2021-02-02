use std::fs::{read_dir, File};
use std::io::{self, BufRead, };
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use serde_derive::{Deserialize, Serialize};

static LOG_DIR: &str = "."; // "C:/Projects/log_parser/";


#[derive(Serialize, Deserialize)]
struct Log {
    text: String
}

fn main() {
    match read_dir(LOG_DIR) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(entries) => {
            for entry in entries {
                let path = entry.unwrap().path();
                if path.is_file() && path.extension().unwrap_or_default() == "log" {
                    println!("File :: {:?}, [{:?}]", path, path.join(".txt"));
                    let file = File::create(path.with_extension("log.txt")).expect("Can't create output file");
                    let mut f = BufWriter::new(file);
                    
                    if let Ok(lines) = read_lines(path) {
                        for line in lines {
                            if let Ok(log_line) = line {
                                let log: Log = serde_json::from_str(&log_line).expect("JSON was not well-formatted");
                                writeln!(f, "{}", log.text.trim());
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
