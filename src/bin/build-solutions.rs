use std::{
    collections::{BTreeSet, HashMap, HashSet},
    error::Error,
    io::Write,
    process,
};

use full_alphabe::{parse_input_ouput_args, Graph};

fn main() -> Result<(), Box<dyn Error>> {
    let input_output = match parse_input_ouput_args() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(-1)
        }
    };

    println!("Reading and parsing...");
    let mut graph: Graph = HashMap::new();
    {
        for line in input_output.input_lines()?.flatten() {
            let mut key_and_values = line.trim().split(':');
            let key = match key_and_values.next() {
                Some(k) => k,
                None => {
                    eprintln!("Invalid line {line}");
                    continue;
                }
            };
            let values = match key_and_values.next() {
                Some(v) => v,
                None => {
                    eprintln!("Invalid line {line}");
                    continue;
                }
            };
            let key: u32 = key.parse()?;
            let values: Vec<u32> = values.split(',').flat_map(|s| s.parse()).collect();
            graph.insert(key, values);
        }
    }

    let mut work: Vec<(Vec<u32>, u32)> = Vec::new();
    let mut solutions: HashSet<BTreeSet<u32>> = HashSet::new();
    let mut completed: HashSet<u32> = HashSet::new();

    for key in graph.keys() {
        let w = (vec![*key], *key);
        work.push(w);
    }

    while let Some(w) = work.pop() {
        let (path, path_bitmap) = w;
        if path.len() == 5 {
            solutions.insert(BTreeSet::from_iter(path));
            continue;
        }
        let current = path.last().unwrap(); // This will always be non empty
        if path.len() == 1 {
            println!("Started processing graph from {current}");
            completed.insert(*current);
        }
        let candidates = graph.get(current).unwrap();
        for candidate in candidates.iter().filter(|c| !completed.contains(c)) {
            if candidate & path_bitmap == 0 {
                let mut next = path.clone();
                next.push(*candidate);
                let w = (next, candidate | path_bitmap);
                work.push(w);
            }
        }
    }

    let mut output = input_output.output_buffer()?;
    for solution in solutions {
        let as_strings: Vec<String> = solution.iter().map(|b| b.to_string()).collect();
        writeln!(output, "{}", as_strings.join(","))?;
    }

    Ok(())
}
