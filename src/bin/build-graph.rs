use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::Write,
    process,
};

use full_alphabe::{parse_input_ouput_args, word_to_bitmap, Graph};

fn main() -> Result<(), Box<dyn Error>> {
    let input_output = match parse_input_ouput_args() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(-1)
        }
    };
    println!("Reading and parsing...");
    let mut words = HashSet::new();
    {
        for line in input_output.input_lines()?.flatten() {
            match word_to_bitmap(line.trim()) {
                Ok(b) => words.insert(b),
                Err(_e) => false,
            };
        }
    }

    println!("Building map from {} distinct word bitmaps...", words.len());
    let mut graph: Graph = HashMap::new();
    let ten_percent = words.len() / 10;
    let mut progress = 0;
    for i in &words {
        for j in &words {
            if i & j != 0 {
                continue;
            }
            if i > j {
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
    let mut output = input_output.output_buffer()?;
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
