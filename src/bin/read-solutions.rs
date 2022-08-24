use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process,
};

use full_alphabe::word_to_bitmap;

fn parse_args() -> Result<(PathBuf, PathBuf), &'static str> {
    let mut args = env::args();
    args.next();

    let solutions = match args.next() {
        Some(input) => input,
        None => return Err("solutions file required"),
    };

    let words = match args.next() {
        Some(input) => input,
        None => return Err("words file required"),
    };

    Ok((
        Path::new(&solutions).to_path_buf(),
        Path::new(&words).to_path_buf(),
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (solutions_path, words_path) = match parse_args() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(-1)
        }
    };

    println!("Reading and parsing solutions...");
    let mut solutions: Vec<Vec<u32>> = Vec::new();
    {
        let raw_contents = File::open(solutions_path)?;
        let lines = BufReader::new(raw_contents).lines();

        for line in lines.flatten() {
            let values: Vec<u32> = line.trim().split(',').flat_map(|s| s.parse()).collect();
            solutions.push(values);
        }
    }

    println!("Reading and parsing words...");
    let mut words: HashMap<u32, Vec<String>> = HashMap::new();
    {
        let raw_contents = File::open(words_path)?;
        let lines = BufReader::new(raw_contents).lines();

        for line in lines.flatten() {
            let word = line.trim().to_owned();
            match word_to_bitmap(&word) {
                Ok(b) => words.entry(b).or_default().push(word),
                Err(_e) => (),
            };
        }
    }

    println!("Determining the {} solutions...", solutions.len());
    for solution in solutions {
        for bitmap in solution {
            print!(
                "{} ",
                words
                    .get(&bitmap)
                    .map(|w| w.join("/"))
                    .unwrap_or_else(|| "UNKNOWN".to_string())
            );
        }
        println!();
    }

    Ok(())
}
