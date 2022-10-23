use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process,
};

use full_alphabe::{word_to_bitmap, words::combinations};

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

    println!("Reading and parsing words...");
    let mut words: HashMap<u32, Vec<&str>> = HashMap::new();
    {
        let raw_contents = File::open(words_path)?;
        let lines = BufReader::new(raw_contents).lines();

        for line in lines {
            if let Ok(word) = line {
                let w = word.trim();
                match word_to_bitmap(&word) {
                    Ok(b) => words.entry(b).or_default().push(&w.to_owned()),
                    Err(_e) => (),
                };
            }
        }
    }

    println!("Reading and parsing solutions...");
    let mut solutions: Vec<Vec<&str>> = Vec::new();
    {
        let raw_contents = File::open(solutions_path)?;
        let lines = BufReader::new(raw_contents).lines();

        for line in lines.flatten() {
            let bitmaps: Result<Vec<u32>, _> = line.split(',').map(|s| s.parse::<u32>()).collect();
            if bitmaps.is_err() {
                eprintln!("Found invalid solution {line}: bad bitmap");
                continue;
            }
            let solution: Option<Vec<Vec<&str>>> = bitmaps
                .unwrap()
                .iter()
                .map(|b| words.get(b).map(|w| w.to_owned()))
                .collect();
            if solution.is_none() {
                eprintln!("Found invalid solution {line}: missing word");
                continue;
            }
            solutions.extend(combinations(&solution.unwrap()));
        }
    }

    println!("Found {} solutions...", solutions.len());
    println!("---");
    for solution in solutions {
        println!("{}", solution.join(" "));
    }

    Ok(())
}
