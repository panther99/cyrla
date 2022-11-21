use std::collections::HashMap;
use std::str;

use lazy_static::lazy_static;
use trie_rs::{Trie, TrieBuilder};

use super::{Converter, ConverterConfig};
use crate::constants::{CYRILLIC_LAT, LAT_CYRILLIC};

lazy_static! {
    static ref LAT_CYR_MAP: HashMap<char, char> = HashMap::from(LAT_CYRILLIC);
    static ref CYR_LAT_MAP: HashMap<char, &'static str> = HashMap::from(CYRILLIC_LAT);
}

trait CharSlice {
    fn char_slice(&self, begin: usize, end: usize) -> &Self;
}

impl CharSlice for str {
    fn char_slice<'a>(&'a self, begin: usize, end: usize) -> &'a str {
        let mut begin_byte = self.len();
        let mut end_byte = self.len();

        let end_position = if end == 0 { 1 } else { end };

        for (i, (byte_index, _)) in self.char_indices().enumerate() {
            if i == begin {
                begin_byte = byte_index;
            } else if i == end_position {
                end_byte = byte_index;
                break;
            }
        }

        unsafe { self.get_unchecked(begin_byte..end_byte) }
    }
}

fn search_prefix(trie: &Trie<u8>, prefix: &str) -> Vec<Vec<u8>> {
    trie.predictive_search(prefix)
}

fn get_matching_results<'a>(results: &'a Vec<Vec<u8>>, lowercase_word: &'a str) -> Vec<&str> {
    unsafe {
        results
            .iter()
            .map(|u8s| str::from_utf8_unchecked(u8s))
            .filter(|s| s.len() <= lowercase_word.len() && lowercase_word.starts_with(s))
            .collect()
    }
}

impl Converter {
    pub fn new(words: &Vec<&str>, config: ConverterConfig) -> Converter {
        let mut trie_builder = TrieBuilder::new();

        for word in words {
            trie_builder.push(word);
        }

        Converter {
            dictionary: trie_builder.build(),
            config,
        }
    }

    pub fn cyr_to_lat(&self, input: &str) -> String {
        let mut converted = String::new();
        let mut iter = input.chars().peekable();

        while let Some(&c) = iter.peek() {
            match CYR_LAT_MAP.get(&c) {
                Some(&letter) => {
                    converted.push_str(letter);
                }
                None => {
                    converted.push(c);
                }
            }

            iter.next();
        }

        converted
    }

    pub fn lat_to_cyr(&self, input: &str) -> String {
        let mut converted = String::new();
        let words: Vec<&str> = input.split(" ").into_iter().collect();

        for (i, word) in words.iter().enumerate() {
            converted.push_str(&self.lat_to_cyr_word(word));

            if i != words.len() - 1 {
                converted.push(' ');
            }
        }

        converted
    }

    fn lat_to_cyr_word(&self, input: &str) -> String {
        let mut current_position: usize = 0;
        let mut converted = String::new();
        let lowercase_word = &input.to_lowercase();
        let chars: Vec<char> = input.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if i < current_position {
                continue;
            }

            let next_char = chars.get(i + 1);
            let current_word_segment = lowercase_word.char_slice(0, current_position);

            match &c {
                'n' => match next_char {
                    Some(&'j') => {
                        if !self.is_prefix_in_dictionary(current_word_segment, lowercase_word) {
                            converted.push('њ');
                            current_position += 2;
                            continue;
                        }

                        converted.push_str("нј");
                        current_position += 1;
                    }
                    _ => converted.push('н'),
                },
                'N' => match next_char {
                    Some(&'j') | Some(&'J') => {
                        if !self.is_prefix_in_dictionary(current_word_segment, lowercase_word) {
                            converted.push('Њ');
                            current_position += 2;
                            continue;
                        }

                        converted.push('Н');
                        converted.push(*LAT_CYR_MAP.get(next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    _ => converted.push('Н'),
                },
                'l' => match next_char {
                    Some(&'j') => {
                        converted.push('љ');
                        current_position += 2;
                        continue;
                    }
                    _ => converted.push('л'),
                },
                'L' => match next_char {
                    Some(&'j') | Some(&'J') => {
                        converted.push('Љ');
                        current_position += 2;
                        continue;
                    }
                    _ => converted.push('Л'),
                },
                'd' => match next_char {
                    Some(&'j') => {
                        if self.config.dj_conversion_enabled
                            && !self.is_prefix_in_dictionary(current_word_segment, lowercase_word)
                        {
                            converted.push('ђ');
                            current_position += 2;
                            continue;
                        }

                        converted.push('д');
                        converted.push(*LAT_CYR_MAP.get(next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    Some(&'ž') => {
                        if !self.is_prefix_in_dictionary(current_word_segment, lowercase_word) {
                            converted.push('џ');
                            current_position += 2;
                            continue;
                        }

                        converted.push_str("дж");
                        current_position += 1;
                    }
                    Some(&'z') => {
                        if self.config.dz_conversion_enabled {
                            converted.push('џ');
                            current_position += 2;
                            continue;
                        }

                        converted.push_str("dz");
                        current_position += 1;
                    }
                    _ => converted.push('д'),
                },
                'D' => match next_char {
                    Some(&'j') | Some(&'J') => {
                        if self.config.dj_conversion_enabled
                            && !self.is_prefix_in_dictionary(current_word_segment, lowercase_word)
                        {
                            converted.push('Ђ');
                            current_position += 2;
                            continue;
                        }

                        converted.push('Д');
                        converted.push(*LAT_CYR_MAP.get(next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    Some(&'ž') | Some(&'Ž') => {
                        if !self.is_prefix_in_dictionary(current_word_segment, lowercase_word) {
                            converted.push('Џ');
                            current_position += 2;
                            continue;
                        }

                        converted.push('Д');
                        converted.push(*LAT_CYR_MAP.get(next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    Some(&'z') | Some(&'Z') => {
                        if self.config.dz_conversion_enabled
                            && !self.is_prefix_in_dictionary(current_word_segment, lowercase_word)
                        {
                            converted.push('Џ');
                            current_position += 2;
                            continue;
                        }

                        converted.push('Д');
                        converted.push(*LAT_CYR_MAP.get(next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    _ => converted.push('Д'),
                },
                _ => match LAT_CYR_MAP.get(c) {
                    Some(&letter) => converted.push(letter),
                    None => converted.push(*c),
                },
            }

            current_position += 1;
        }

        converted
    }

    fn is_prefix_in_dictionary(&self, prefix: &str, lowercase_word: &str) -> bool {
        let results = search_prefix(&self.dictionary, prefix);

        if results.is_empty() {
            return false;
        }

        let matching_results = get_matching_results(&results, lowercase_word);

        if matching_results.is_empty() {
            return false;
        }

        true
    }
}
