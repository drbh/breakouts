use breakouts::{split_vec_to_index, string_to_static_str, HasCool};
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub attrs: PersonAttrs,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PersonAttrs {
    pub is_cool: bool,
    pub age: usize,
    pub comprehension: usize,
    pub fav_color: String,
}

pub fn make_me() -> Person {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..80);

    Person {
        name: "David".into(),
        attrs: PersonAttrs {
            is_cool: true,
            age: die.sample(&mut rng),
            comprehension: die.sample(&mut rng),
            fav_color: "Black".into(),
        },
    }
}

impl HasCool for Person {
    fn attributes(&self) -> Vec<(&str, &str)> {
        return [
            ("is_cool", &string_to_static_str(self.attrs.is_cool)[..]),
            ("name", &self.name[..]),
            ("age", &string_to_static_str(self.attrs.age)[..]),
            ("fav_color", &self.attrs.fav_color[..]),
        ]
        .to_vec();
    }
}

fn main() {
    // Make some dummy data - like 1M
    let mut vec = Vec::new();
    for _ in 0..1_000_000 {
        // for _ in 0..50_000 {
        vec.push(make_me());
    }

    // Index this data and time it
    let start = Instant::now();
    let res = split_vec_to_index::<Person>(&vec);
    let duration = start.elapsed();
    println!("Time elapsed while indexing: {:?}", duration);

    // Now we wanna save it - as bytes to disk
    let start = Instant::now();
    let encoded: Vec<u8> = bincode::serialize(&res).unwrap();
    let mut file = File::create("test").unwrap();
    file.write_all(&encoded[..]).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed while serialize: {:?}", duration);

    // Now we wanna read those bytes
    let start = Instant::now();
    let mut file = File::open("test").unwrap();
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).expect("failed to read");
    let res: HashMap<(&str, &str), Vec<usize>> = bincode::deserialize(&encoded[..]).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed while deserialize: {:?}", duration);

    // Last we want to check which of those people attributes meet some criteria
    let start = Instant::now();
    for (k, _v) in res.iter() {
        if k.0 == "is_cool" && k.1 == "true" {
            // println!("name::david {:?}: {:?}", k, _v); // print match and indexes
        }
        if k.0 == "name" && k.1 == "David" {
            // println!("name::david {:?}: {:?}", k, _v); // print match and indexes
        }
        if k.0 == "age" && k.1.to_string().parse::<usize>().unwrap() >= 60 {
            // println!("age::sixty {:?}: {:?}", k, _v); // print match and indexes
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed while grouping: {:?}", duration);
}
