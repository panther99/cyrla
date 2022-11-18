use trie_rs::Trie;

pub mod converter;
pub mod converter_builder;

pub struct Converter {
    dictionary: Trie<u8>,
}

pub struct ConverterBuilder<'a> {
    dictionary: Vec<&'a str>,
    dj_conversion_enabled: bool,
    dz_conversion_enabled: bool,
    ijekavian_inclusion_enabled: bool,
}
