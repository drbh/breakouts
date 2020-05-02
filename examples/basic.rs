use breakouts::{split_vec_to_index, HasCool};
use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub attrs: PersonAttrs,
}

#[derive(Debug, PartialEq)]
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
    fn bool_val(&self) -> bool {
        self.attrs.is_cool
    }
    fn number_val(&self) -> usize {
        self.attrs.age
    }
    fn string_val(&self) -> String {
        self.attrs.fav_color.to_string()
    }
}

fn main() {
    let mut vec = Vec::new();

    for _ in 0..1_000_000 {
        vec.push(make_me());
    }

    let start = Instant::now();
    let res = split_vec_to_index::<Person>(&vec);
    let duration = start.elapsed();
    println!("Time elapsed while indexing: {:?}", duration);

    let start = Instant::now();
    // let mut myresults: Vec<String> = Vec::new();
    for (k, _v) in res.iter() {
        // things we want to lookup
        if k.0 == "number_val" && k.1.clone().as_usize() >= 60 {
            // something did match
            // println!("number_val {:?}: {:?}", k, _v);
            // myresults.push(format!("number_val {:?}: {:?}", k, _v))
        }
        if k.0 == "string_val" && k.1.clone().as_string() == String::from("Red") {
            // something did match
            // println!("string_val {:?}: {:?}", k, _v);
            // myresults.push(format!("string_val {:?}: {:?}", k, _v))
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed while grouping: {:?}", duration);
}

use std::time::Instant;
