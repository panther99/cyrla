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
            dj_conversion_enabled: true,
            dz_conversion_enabled: true,
            ijekavian_inclusion_enabled: true,
        }
    }

    /// Adds given prefixes to the dictionary which will be used by the `Converter`
    pub fn extend_dictionary(&mut self, prefixes: &mut Vec<&'a str>) -> &mut ConverterBuilder<'a> {
        self.dictionary.append(prefixes);
        self
    }

    /// Disables conversion from `dj` to `ђ` for words which don't have prefix in the dictionary
    pub fn disable_dj_conversion(&mut self) -> &mut ConverterBuilder<'a> {
        self.dj_conversion_enabled = false;
        self
    }

    /// Disables conversion from `dz` to `џ` for words which don't have prefix in the dictionary
    pub fn disable_dz_conversion(&mut self) -> &mut ConverterBuilder<'a> {
        self.dz_conversion_enabled = false;
        self
    }

    /// Disables both, conversions from `dj` to `ђ` and from `dz` to `џ`
    pub fn disable_bald_latin(&mut self) -> &mut ConverterBuilder<'a> {
        self.disable_dj_conversion();
        self.disable_dz_conversion();
        self
    }

    /// Disables including ijekavian words (otherwise they won't have `dj` converted to `ђ` if
    /// dj conversion is enabled)
    pub fn disable_ijekavian_inclusion(&mut self) -> &mut ConverterBuilder<'a> {
        self.ijekavian_inclusion_enabled = false;
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
        };

        Converter::new(&self.dictionary, config)
    }
}
