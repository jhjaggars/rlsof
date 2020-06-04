extern crate rlsof;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rlsof::map_line;
use std::collections::HashMap;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("lsof0.output") {
        let x: Vec<_> = lines.map(|line| line.unwrap()).map(|line| {
            let mut rec: HashMap<&str, String> = HashMap::new();
            map_line(&line, &mut rec);
            rec
        }).collect();
        println!("{:?}", x);
    }
}
