use std::str;
use trie_rs::{Trie, TrieBuilder};
use super::{Converter, ConverterConfig};

trait CharSlice {
    fn char_slice(&self, begin: usize, end: usize) -> &Self;
}

impl CharSlice for str {
    fn char_slice(&self, begin: usize, end: usize) -> &str {
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

fn lat_letter_to_cyr(letter: char) -> Option<char> {
    match letter {
        'A' => Some('А'),
        'B' => Some('Б'),
        'V' => Some('В'),
        'G' => Some('Г'),
        'D' => Some('Д'),
        'Đ' => Some('Ђ'),
        'E' => Some('Е'),
        'Ž' => Some('Ж'),
        'Z' => Some('З'),
        'I' => Some('И'),
        'J' => Some('Ј'),
        'K' => Some('К'),
        'L' => Some('Л'),
        'M' => Some('М'),
        'N' => Some('Н'),
        'O' => Some('О'),
        'P' => Some('П'),
        'R' => Some('Р'),
        'S' => Some('С'),
        'T' => Some('Т'),
        'Ć' => Some('Ћ'),
        'U' => Some('У'),
        'F' => Some('Ф'),
        'H' => Some('Х'),
        'C' => Some('Ц'),
        'Č' => Some('Ч'),
        'Š' => Some('Ш'),
        'a' => Some('а'),
        'b' => Some('б'),
        'v' => Some('в'),
        'g' => Some('г'),
        'd' => Some('д'),
        'đ' => Some('ђ'),
        'e' => Some('е'),
        'ž' => Some('ж'),
        'z' => Some('з'),
        'i' => Some('и'),
        'j' => Some('ј'),
        'k' => Some('к'),
        'l' => Some('л'),
        'm' => Some('м'),
        'n' => Some('н'),
        'o' => Some('о'),
        'p' => Some('п'),
        'r' => Some('р'),
        's' => Some('с'),
        't' => Some('т'),
        'ć' => Some('ћ'),
        'u' => Some('у'),
        'f' => Some('ф'),
        'h' => Some('х'),
        'c' => Some('ц'),
        'č' => Some('ч'),
        'š' => Some('ш'),
        _   => None,
    }
}

fn cyr_letter_to_lat(letter: char) -> Option<&'static str> {
    match letter {
        'А' => Some("A"),
        'Б' => Some("B"),
        'В' => Some("V"),
        'Г' => Some("G"),
        'Д' => Some("D"),
        'Ђ' => Some("Đ"),
        'Е' => Some("E"),
        'Ж' => Some("Ž"),
        'З' => Some("Z"),
        'И' => Some("I"),
        'Ј' => Some("J"),
        'К' => Some("K"),
        'Л' => Some("L"),
        'Љ' => Some("Lj"),
        'М' => Some("M"),
        'Н' => Some("N"),
        'Њ' => Some("Nj"),
        'О' => Some("O"),
        'П' => Some("P"),
        'Р' => Some("R"),
        'С' => Some("S"),
        'Т' => Some("T"),
        'Ћ' => Some("Ć"),
        'У' => Some("U"),
        'Ф' => Some("F"),
        'Х' => Some("H"),
        'Ц' => Some("C"),
        'Ч' => Some("Č"),
        'Џ' => Some("Dž"),
        'Ш' => Some("Š"),
        'а' => Some("a"),
        'б' => Some("b"),
        'в' => Some("v"),
        'г' => Some("g"),
        'д' => Some("d"),
        'ђ' => Some("đ"),
        'е' => Some("e"),
        'ж' => Some("ž"),
        'з' => Some("z"),
        'и' => Some("i"),
        'ј' => Some("j"),
        'к' => Some("k"),
        'л' => Some("l"),
        'љ' => Some("lj"),
        'м' => Some("m"),
        'н' => Some("n"),
        'њ' => Some("nj"),
        'о' => Some("o"),
        'п' => Some("p"),
        'р' => Some("r"),
        'с' => Some("s"),
        'т' => Some("t"),
        'ћ' => Some("ć"),
        'у' => Some("u"),
        'ф' => Some("f"),
        'х' => Some("h"),
        'ц' => Some("c"),
        'ч' => Some("č"),
        'џ' => Some("dž"),
        'ш' => Some("š"),
        _   => None, 
    }
}

fn search_prefix(trie: &Trie<u8>, prefix: &str) -> Vec<Vec<u8>> {
    trie.predictive_search(prefix)
}

fn get_matching_results<'a>(results: &'a [Vec<u8>], lowercase_word: &'a str) -> Vec<&str> {
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
            match cyr_letter_to_lat(c) {
                Some(letter) => {
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
        let words: Vec<&str> = input.split(' ').into_iter().collect();

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
                        converted.push(lat_letter_to_cyr(*next_char.unwrap()).unwrap());
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
                        converted.push(lat_letter_to_cyr(*next_char.unwrap()).unwrap());
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
                        converted.push(lat_letter_to_cyr(*next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    Some(&'ž') | Some(&'Ž') => {
                        if !self.is_prefix_in_dictionary(current_word_segment, lowercase_word) {
                            converted.push('Џ');
                            current_position += 2;
                            continue;
                        }

                        converted.push('Д');
                        converted.push(lat_letter_to_cyr(*next_char.unwrap()).unwrap());
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
                        converted.push(lat_letter_to_cyr(*next_char.unwrap()).unwrap());
                        current_position += 1;
                    }
                    _ => converted.push('Д'),
                },
                _ => match lat_letter_to_cyr(*c) {
                    Some(letter) => converted.push(letter),
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
