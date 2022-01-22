use crate::engine::words::DICTIONARY_WORDS;
use rand::Rng;
use std::collections::HashMap;
use trie_rs::{Trie, TrieBuilder};

pub fn dictionary() -> Trie<u8> {
    let mut dict = TrieBuilder::new();
    for w in DICTIONARY_WORDS {
        dict.push(w);
    }
    dict.build()
}

pub fn get_random_word() -> &'static str {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..DICTIONARY_WORDS.len());

    DICTIONARY_WORDS[i]
}

pub fn build_letter_counts(word: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in word.chars() {
        match counts.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                counts.insert(c, 1);
                ()
            }
        };
    }
    counts
}
