//! Library for two-way conversion between latin and cyrillic script
//!
//! # Getting started
//!
//! Not available for public use yet - this is a development preview.
//!
//! ## Usage
//!
//! ```rust
//! use cyrla::ConverterBuilder;
//!
//! let converter = ConverterBuilder::new().build();
//!
//! // conversion from latin to cyrillic script
//! let cyrillic_text = converter.lat_to_cyr("Konjugacija u genetici je sparivanje homolognih hromozoma u mejozi.");
//! assert_eq!(cyrillic_text, "Конјугација у генетици је спаривање хомологних хромозома у мејози.");
//!
//! // conversion from cyrillic to latin script
//! let latin_text = converter.cyr_to_lat("Ђурђевак је њена омиљена биљка.");
//! assert_eq!(latin_text, "Đurđevak je njena omiljena biljka.");
//! ```
//!
//! You can also configure builder for your needs:
//!
//! ```rust
//! use cyrla::ConverterBuilder;
//!
//! let mut builder = ConverterBuilder::new();
//! let mut prefixes = vec!["kunjar", "abanjar", "sanjar"];
//! let converter = builder
//!     .extend_dictionary(&mut prefixes)
//!     .build();
//!
//! let cyrillic_text = converter.lat_to_cyr("Kunjar, Abanjar i Sanjar sudjelovali su u zadatku.");
//! assert_eq!(cyrillic_text, "Кунјар, Абанјар и Санјар судјеловали су у задатку.");
//! ```
//!
//! You can find all available options in the `ConverterBuilder` documentation.

pub mod constants;
pub mod converter;

pub use converter::{Converter, ConverterBuilder};

#[cfg(test)]
mod tests {
    use crate::ConverterBuilder;

    #[test]
    fn it_properly_converts_latin_to_cyrillic() {
        let converter = ConverterBuilder::new().build();
        let cyrillic_text =
            converter.lat_to_cyr("'Oće centrala da pogreši jednom, ali ne sto puta!");

        assert_eq!(
            "'Оће централа да погреши једном, али не сто пута!",
            cyrillic_text
        );
    }

    #[test]
    fn it_properly_converts_cyrillic_to_latin() {
        let converter = ConverterBuilder::new().build();
        let latin_text = converter.cyr_to_lat("Ђурђевак је њена омиљена биљка.");

        assert_eq!("Đurđevak je njena omiljena biljka.", latin_text);
    }

    #[test]
    fn it_properly_converts_literal_prefixes() {
        let converter = ConverterBuilder::new().build();
        let cyrillic_text = converter
            .lat_to_cyr("Konjugacija u genetici je sparivanje homolognih hromozoma u mejozi.");

        assert_eq!(
            "Конјугација у генетици је спаривање хомологних хромозома у мејози.",
            cyrillic_text
        );
    }

    #[test]
    fn it_properly_converts_words_added_to_dictionary() {
        let mut builder = ConverterBuilder::new();
        let mut words = vec!["kunjar", "abanjar", "sanjar"];
        let converter = builder.extend_dictionary(&mut words).build();
        let cyrillic_text =
            converter.lat_to_cyr("Abanjar, Kunjar i Sanjar su sudjelovali u zadatku.");

        assert_eq!(
            "Абанјар, Кунјар и Санјар су судјеловали у задатку.",
            cyrillic_text
        );
    }

    #[test]
    fn it_disables_ijekavian_inclusion() {
        let mut builder = ConverterBuilder::new();
        let converter = builder.disable_ijekavian_inclusion().build();

        let cyrillic_text = converter.lat_to_cyr("Nensi, gdje su djeca?");

        assert_eq!("Ненси, гђе су ђеца?", cyrillic_text);
    }

    #[test]
    fn it_disables_dj_conversion() {
        let mut builder = ConverterBuilder::new();
        let converter = builder.disable_dj_conversion().build();

        let cyrillic_text = converter.lat_to_cyr("Djor dji ja");

        assert_eq!("Дјор дји ја", cyrillic_text);
    }
}
