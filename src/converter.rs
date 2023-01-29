use trie_rs::Trie;

mod converter;
mod converter_builder;

pub struct ConverterConfig<'a> {
    dj_conversion_enabled: bool,
    dz_conversion_enabled: bool,
    ignored_latin_words: &'a Vec<&'a str>,
}

pub struct Converter<'a> {
    dictionary: Trie<u8>,
    ignored_latin_words: Trie<u8>,
    config: ConverterConfig<'a>,
}

pub struct ConverterBuilder<'a> {
    dictionary: Vec<&'a str>,
    ignored_latin_words: Vec<&'a str>,
    dj_conversion_enabled: bool,
    dz_conversion_enabled: bool,
    ijekavian_inclusion_enabled: bool,
}
