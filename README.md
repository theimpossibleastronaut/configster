[![Build Status](https://travis-ci.com/theimpossibleastronaut/configster.svg?branch=trunk)](https://travis-ci.com/theimpossibleastronaut/configster)
[![crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/configster.svg
[crates-url]: https://crates.io/crates/configster

# configster

Rust library for parsing configuration files

## Config file format

The 'option' can be any string with no whitespace.

```ini
arbitrary_option = false
max_users = 30
```

The value for an option following the equal sign may have "attributes"
that are separated by a delimiter. The delimiter is specified when
calling parse_file():

```rust
parse_file("./config_test.conf", ',')
```

```ini
option = Blue, light, shiny
# option = nothing, void, empty, commented_out
```

An option is not required to be followed by a value. It can be used to disable a default feature.

```ini
FeatureOff
```

## API

Calling parse_file() will return a single vector containing a struct
(OptionProperties) for each option line in the config file. The
attributes for a value are stored in a vector within the "Value"
struct.

```rust
#[derive(Debug, PartialEq)]
pub struct Value {
    pub primary: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct OptionProperties {
    pub option: String,
    pub value: Value,
}
```

## Example Code

```rust
use std::io;

fn main() -> Result<(), io::Error> {

    let config_vec = configster::parse_file("./config_test.conf", ',')?;

    for i in &config_vec {
        println!("Option:'{}' | value '{}'", i.option, i.value.primary);

        for j in &i.value.attributes {
            println!("attr:'{}`", j);
        }
        println!();
    }
    Ok(())
}
```

See [docs.rs/configster/](https://docs.rs/configster/)
for generated API documentation.

## Contributing

See [CONTRIBUTING.md](https://github.com/theimpossibleastronaut/configster/CONTRIBUTING.md)
