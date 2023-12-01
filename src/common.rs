use std::{path::Path, fs::File, io::{self, BufRead}};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_lines<F>(file_path: &str, mut func: F) where F: FnMut(String) {
  let file_name = String::from("input/") + file_path;
  let file_path = Path::new(&file_name);

  if let Ok(lines) = read_lines(file_path) {
    for line in lines {
      if let Ok(l) = line {
        func(l)
      }
    }
  }
}