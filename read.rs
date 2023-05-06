type Vertex = usize;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Vec<(Vertex, Vertex)> {
    let mut result: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        if v.len() != 1{
            let z = v[1].parse::<Vertex>().unwrap();
            let x = v[0].parse::<Vertex>().unwrap();
            result.push((x, z));
        }else{
            continue;
        }
    }
    return  result;
}

pub fn read_label(path: &str) -> Vec<(Vertex, Vertex)> {
    let mut result: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        if v.len() != 1{
            let z = v[1].parse::<Vertex>().unwrap();
            let x = v[0].parse::<Vertex>().unwrap();
            result.push((z, x));
        }else{
            continue;
        }
    }
    return  result;
}