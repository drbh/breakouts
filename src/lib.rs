use std::collections::HashMap;

pub trait HasCool {
    fn attributes(&self) -> Vec<(&str, &str)>;
}

pub fn string_to_static_str<T: std::string::ToString>(s: T) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

pub fn split_vec_to_index<T: HasCool>(people: &Vec<T>) -> HashMap<(&str, &str), Vec<usize>> {
    let mut index = HashMap::new();
    for (count, person_in_question) in people.iter().enumerate() {
        for attr in person_in_question.attributes() {
            let index_list = index.entry((attr.0, attr.1)).or_insert(Vec::new());
            index_list.push(count);
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;
}
