use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Lines},
    path::PathBuf,
};

pub type Graph = HashMap<u32, Vec<u32>>;

pub struct InputOutput {
    input: PathBuf,
    output: PathBuf,
}

impl InputOutput {
    pub fn input_lines(&self) -> Result<Lines<BufReader<File>>, io::Error> {
        File::open(&self.input)
            .map(BufReader::new)
            .map(|r| r.lines())
    }

    pub fn output_buffer(&self) -> Result<BufWriter<File>, io::Error> {
        File::create(&self.output).map(BufWriter::new)
    }
}

pub fn parse_input_ouput_args() -> Result<InputOutput, &'static str> {
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

    Ok(InputOutput {
        input: PathBuf::from(input),
        output: PathBuf::from(output),
    })
}

#[derive(PartialEq, Debug)]
pub enum BitmapError {
    NotUnique(char),
}

pub fn word_to_bitmap(word: &str) -> Result<u32, BitmapError> {
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

#[cfg(test)]
mod tests {}
