use trie_rs::Trie;

mod converter;
pub mod converter_builder;

pub struct ConverterConfig {
    dj_conversion_enabled: bool,
    dz_conversion_enabled: bool,
}

pub struct Converter {
    dictionary: Trie<u8>,
    config: ConverterConfig,
}

pub struct ConverterBuilder<'a> {
    dictionary: Vec<&'a str>,
    dj_conversion_enabled: bool,
    dz_conversion_enabled: bool,
    ijekavian_inclusion_enabled: bool,
}
