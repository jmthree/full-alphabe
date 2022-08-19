use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process,
};

#[derive(PartialEq, Debug)]
enum BitmapError {
    NotUnique(char),
}

fn word_to_bitmap(word: &str) -> Result<u32, BitmapError> {
    let mut bitmap = 0;
    for c in word.chars() {
        let pow = c as u32 - 'a' as u32;
        let index = 1 << pow;
        if bitmap & index != 0 {
            return Err(BitmapError::NotUnique(c));
        }
        bitmap |= index;
    }
    Ok(bitmap)
}

fn parse_args() -> Result<(PathBuf, PathBuf), &'static str> {
    let mut args = env::args();
    args.next();

    let input = match args.next() {
        Some(input) => input,
        None => return Err("input file required"),
    };

    let output = match args.next() {
        Some(output) => output,
        None => return Err("output file required"),
    };

    Ok((
        Path::new(&input).to_path_buf(),
        Path::new(&output).to_path_buf(),
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (input, output) = match parse_args() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(-1)
        }
    };
    println!("Reading and parsing...");
    let mut words = HashSet::new();
    {
        let raw_contents = File::open(input)?;
        let lines = BufReader::new(raw_contents).lines();

        for line in lines.flatten() {
            match word_to_bitmap(line.trim()) {
                Ok(b) => words.insert(b),
                Err(_e) => false,
            };
        }
    }

    println!("Building map from {} distinct word bitmaps...", words.len());
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let ten_percent = words.len() / 10;
    let mut progress = 0;
    for i in &words {
        for j in &words {
            if i & j != 0 {
                continue;
            }
            graph.entry(*i).or_default().push(*j);
        }
        progress += 1;
        if progress % ten_percent == 0 {
            println!("{}0% done", (progress / ten_percent));
        }
    }

    println!("Writing...");
    let mut output = BufWriter::new(File::create(output)?);
    progress = 0;
    for e in graph {
        write!(output, "{}:", e.0)?;
        let mut sep = "";
        for u in e.1 {
            write!(output, "{}{}", sep, u)?;
            sep = ",";
        }
        writeln!(output)?;
        progress += 1;
        if progress % ten_percent == 0 {
            println!("{}0% done", (progress / ten_percent));
            output.flush()?;
        }
    }
    output.flush()?;

    Ok(())
}
