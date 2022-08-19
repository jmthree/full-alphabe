use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Error, Write},
    path::Path,
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

fn main() -> Result<(), Error> {
    // This vector of id to alpha-bitmap only works because we know
    // there's about 14k words in our input
    println!("Reading and parsing...");
    let mut words: Vec<(u16, u32)> = vec![];
    {
        let words_file = Path::new("words_len5.txt");
        let raw_contents = File::open(words_file)?;
        let lines = BufReader::new(raw_contents).lines();

        for (i, line) in lines.flatten().enumerate() {
            match word_to_bitmap(line.trim()) {
                Ok(b) => words.push((i as u16, b)),
                Err(_e) => (),
            }
        }
    }

    println!("Building map from {} words...", words.len());
    let mut graph: HashMap<u16, Vec<u16>> = HashMap::new();
    let ten_percent = words.len() / 10;
    let mut progress = 0;
    for (i, ib) in &words {
        for (j, jb) in &words {
            if ib & jb != 0 {
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
    let mut output = BufWriter::new(File::create(Path::new("graph.txt"))?);
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
