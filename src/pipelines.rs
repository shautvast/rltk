use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};

pub fn count() -> anyhow::Result<()> {
    let mut store: BTreeMap<String, usize> = BTreeMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        for token in line.split(|c: char| c.is_ascii_punctuation() || c.is_whitespace()) {
            let count = store.entry(token.to_owned()).or_insert(0);
            *count += 1;
        }
    }
    for (key, value) in store{
        println!("{}:{}", key,value);
    }
    Ok(())
}

fn hash(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

pub fn create_binary_bow(file: File) {}