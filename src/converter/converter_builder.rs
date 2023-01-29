use super::{Converter, ConverterBuilder, ConverterConfig};
use crate::constants::{IJEKAVIAN_PREFIXES, LITERAL_PREFIXES};

impl<'a> Default for ConverterBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ConverterBuilder<'a> {
    pub fn new() -> ConverterBuilder<'a> {
        ConverterBuilder {
            dictionary: Vec::from(LITERAL_PREFIXES),
            ignored_latin_words: vec![],
            dj_conversion_enabled: false,
            dz_conversion_enabled: false,
            ijekavian_inclusion_enabled: false,
        }
    }

    /// Adds given prefixes to the dictionary which will be used by the `Converter`
    pub fn extend_dictionary(&mut self, prefixes: &mut Vec<&'a str>) -> &mut ConverterBuilder<'a> {
        self.dictionary.append(prefixes);
        self
    }

    /// Adds words which will be skipped during conversion process when converting from latin to cyrillic script
    pub fn add_ignored_latin_words(&mut self, words: &mut Vec<&'a str>) -> &mut ConverterBuilder<'a> {
        self.ignored_latin_words.append(words);
        self
    }

    /// Enables conversion from `dj` to `ђ` for words which don't have prefix in the dictionary
    pub fn enable_dj_conversion(&mut self) -> &mut ConverterBuilder<'a> {
        self.dj_conversion_enabled = true;
        self
    }

    /// Enables conversion from `dz` to `џ` for words which don't have prefix in the dictionary
    pub fn enable_dz_conversion(&mut self) -> &mut ConverterBuilder<'a> {
        self.dz_conversion_enabled = true;
        self
    }

    /// Enables both, conversions from `dj` to `ђ` and from `dz` to `џ`
    pub fn enable_bald_latin(&mut self) -> &mut ConverterBuilder<'a> {
        self.enable_dj_conversion();
        self.enable_dz_conversion();
        self
    }

    /// Enables including ijekavian words (otherwise they won't have `dj` converted to `ђ` if
    /// dj conversion is enabled)
    pub fn enable_ijekavian_inclusion(&mut self) -> &mut ConverterBuilder<'a> {
        self.ijekavian_inclusion_enabled = true;
        self
    }

    pub fn build(&mut self) -> Converter {
        if self.ijekavian_inclusion_enabled {
            for prefix in IJEKAVIAN_PREFIXES {
                let _ = &self.dictionary.push(prefix);
            }
        }

        let config = ConverterConfig {
            dj_conversion_enabled: self.dj_conversion_enabled,
            dz_conversion_enabled: self.dz_conversion_enabled,
            ignored_latin_words: &self.ignored_latin_words,
        };

        Converter::new(&self.dictionary, config)
    }
}
