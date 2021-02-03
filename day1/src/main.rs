use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Eq, Hash)]
struct Accumulate {
    number_of_parts: u8,
    sum: i64,
    product: i64,
}

fn task1(filename : &str) {
    let f = File::open(filename).unwrap();
    let br = BufReader::new(f);

    let mut visited = HashSet::<i32>::new();

    for line in br.lines() {
        if let Ok(val) = line.unwrap().parse::<i32>() {
            let complement = 2020 - val;
            if visited.contains(&complement) {
                println!("{:?}", complement*val);
            }
            visited.insert(val);
        }
    }
}

fn task2(filename : &str) {
    let f = File::open(filename).unwrap();
    let br = BufReader::new(f);

    let mut visited = HashSet::<Accumulate>::new();
    visited.insert(Accumulate {
        number_of_parts: 0,
        sum: 0,
        product: 1,
    });

    for line in br.lines() {
        if let Ok(val) = line.unwrap().parse::<i32>() {
            let newVisited: HashSet<Accumulate> = visited
                .iter()
                .map(|x| Accumulate {
                    number_of_parts: x.number_of_parts + 1,
                    sum: x.sum + val as i64,
                    product: x.product * val as i64,
                })
                .filter(|x| x.number_of_parts <= 3 && x.sum <= 2020)
                .collect();

            for toAdd in newVisited {
                if toAdd.number_of_parts == 3 && toAdd.sum == 2020 {
                    println!("result = {:?}", toAdd.product);
                }
                visited.insert(toAdd);
            }
        }
    }

    println!("visited = {:?}", visited.len());
}




fn main() {
    let args: Vec<String> = env::args().collect();

    task1(&args[1]);
    task2(&args[1]);
}