use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub fn read(path: &Path) -> Vec<Vec<char>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect()
}

pub fn write(file: &Path, lines: Vec<Vec<char>>) {
    let mut file = File::create(file).unwrap();
    for characters in lines {
        let line: String = characters.iter().collect();
        file.write_all((line + "\n").as_bytes()).unwrap();
    }

    file.flush().unwrap();
}
