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

    println!("Determining the solutions...");
    println!("---");
    'solutions: for solution in solutions {
        let mut solution_words: Vec<Vec<String>> = Vec::new();
        for bitmap in solution {
            if let Some(words) = words.get(&bitmap) {
                if solution_words.is_empty() {
                    for word in words {
                        solution_words.push(vec![word.to_string()]);
                    }
                }
                let mut new_words: Vec<Vec<String>> = Vec::new();
                for word in words {
                    for existing in &solution_words {
                        let mut new = existing.clone();
                        new.push(word.to_string());
                        new_words.push(new);
                    }
                }
                solution_words = new_words;
            } else {
                eprintln!("No word found for bitmap {bitmap}. Skipping solution.");
                continue 'solutions;
            }
        }
        for words in solution_words {
            println!("{}", words.join(", "));
        }
    }

    Ok(())
}
