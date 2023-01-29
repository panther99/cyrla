# cyrla

Library for two-way conversion between latin and cyrillic script, written in Rust.

## Getting started

To use cyrla, add following to your `Cargo.toml` file:

```toml
[dependencies]
cyrla = "0.1"
```

## Usage

```rust
use cyrla::ConverterBuilder;

let converter = ConverterBuilder::new().build();

// conversion from latin to cyrillic script
let cyrillic_text = converter.lat_to_cyr("Konjugacija u genetici je sparivanje homolognih hromozoma u mejozi.");
assert_eq!(cyrillic_text, "Конјугација у генетици је спаривање хомологних хромозома у мејози.");

// conversion from cyrillic to latin script
let latin_text = converter.cyr_to_lat("Ђурђевак је њена омиљена биљка.");
assert_eq!(latin_text, "Đurđevak je njena omiljena biljka.");
```

You can also configure builder for your needs:

```rust
use cyrla::ConverterBuilder;

let mut builder = ConverterBuilder::new();
let mut prefixes = vec!["kunjar", "abanjar", "sanjar"];
let converter = builder
    .extend_dictionary(&mut prefixes)
    .build();

let cyrillic_text = converter.lat_to_cyr("Kunjar, Abanjar i Sanjar sudjelovali su u zadatku.");
assert_eq!(cyrillic_text, "Кунјар, Абанјар и Санјар судјеловали су у задатку.");
```

You can find all available options in the `ConverterBuilder` documentation.

## License

This project is licensed under the terms of MIT license.
