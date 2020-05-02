use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Bool(bool),
    Number(usize),
    String(String),
}

impl Value {
    pub fn as_bool(self) -> bool {
        if let Value::Bool(c) = self {
            c
        } else {
            panic!("Not a Bool Val")
        }
    }

    pub fn as_usize(self) -> usize {
        if let Value::Number(d) = self {
            d
        } else {
            panic!("Not a Number Val")
        }
    }

    pub fn as_string(self) -> String {
        if let Value::String(d) = self {
            d
        } else {
            panic!("Not a String Val")
        }
    }
}

pub trait HasCool {
    fn bool_val(&self) -> bool;
    fn number_val(&self) -> usize;
    fn string_val(&self) -> String;
}

pub fn split_vec_to_index<T: std::marker::Sync + HasCool>(
    people: &Vec<T>,
) -> HashMap<(&str, Value), Vec<usize>> {
    let mut index = HashMap::new();

    for (count, person_in_question) in people.iter().enumerate() {
        {
            let var = person_in_question.bool_val();
            let key = ("bool_val", Value::Bool(var));
            let index_list = index.entry(key).or_insert(Vec::new());
            index_list.push(count);
        }
        {
            let age = person_in_question.number_val();
            let key = ("number_val", Value::Number(age));
            let index_list = index.entry(key).or_insert(Vec::new());
            index_list.push(count);
        }
        {
            let var = person_in_question.string_val();
            let key = ("string_val", Value::String(var));
            let index_list = index.entry(key).or_insert(Vec::new());
            index_list.push(count);
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;
}
