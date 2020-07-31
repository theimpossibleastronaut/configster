use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/// Returns the library version
#[inline]
pub fn get_ver() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

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

impl OptionProperties {
    fn new(option: String, primary: String, attributes: Vec<String>) -> Self {
        Self {
            option,
            value: Value {
                primary,
                attributes,
            },
        }
    }
}

/// Parses a configuration file. The second parameter sets the delimiter for the
/// attribute list of the primary value. The return value is a vector wrapped in
/// an io::Result type.
///
/// ## Examples
///
/// Config file format:
///
/// ```text
/// ExampleOption = 12
///
/// ExampleOption2 = /home/foo/bar, optional, attribute, list, for, value
///
/// example_option3 = Hello
///
/// # Option = commented_out_using_hashtag
/// ```
///
/// Options Without Values:
///
/// ```text
/// DefaultFeatureFooDisabled
/// ```
///
/// Options With the Same Name:
///
/// ```text
/// color = Green
/// color = Blue
/// color = Black
/// ```
///
/// Accessing the Parsed Data:
///
/// ```
/// use std::io;
///
/// fn main() -> Result<(), io::Error> {
///
///     let config_vec = configster::parse_file("./config_test.conf", ',');
///     if config_vec.is_err() {
///         return io::Result::Err(config_vec.unwrap_err());
///     }
///
///     for i in &config_vec.unwrap() {
///         println!("Option:'{}' | value '{}'", i.option, i.value.primary);
///
///         for j in &i.value.attributes {
///             println!("attr:'{}`", j);
///         }
///         println!();
///     }
///     Ok(())
/// }
/// ```
#[inline]
pub fn parse_file(filename: &str, attr_delimit_char: char) -> io::Result<Vec<OptionProperties>> {
    let file = File::open(filename);
    if file.is_err() {
        return io::Result::Err(file.unwrap_err());
    }

    let reader = BufReader::new(file.unwrap());
    let mut vec: Vec<OptionProperties> = Vec::new();

    // for (line, index) in reader.lines().enumerate() {
    for line in reader.lines() {
        if line.is_err() {
            return io::Result::Err(line.unwrap_err());
        }

        let l = line.unwrap();

        // Parse the line, return the properties
        let (option, primary_value, attr_vec) = parse_line(&l, attr_delimit_char);

        if option.is_empty() {
            continue;
        }

        let opt_props = OptionProperties::new(option, primary_value, attr_vec);
        vec.push(opt_props);

        // Show the line and its number.
        // println!("{}. {}", index + 1, l);
    }
    Ok(vec)
}

/// Returns the properties of the option, derived from
/// a line in the configuration file.
fn parse_line(l: &str, attr_delimit_char: char) -> (String, String, Vec<String>) {
    let line = l.trim();
    if line.is_empty() || line.as_bytes()[0] == b'#' {
        return ("".to_string(), "".to_string(), vec![]);
    }

    let mut i = line.find('=');
    let (mut option, value) = match i.is_some() {
        true => (
            format!("{}", &line[..i.unwrap()].trim()),
            format!("{}", &line[i.unwrap() + 1..].trim()),
        ),
        false => (line.to_string(), String::new()),
    };

    // An Equal sign is required after 'Option'; spaces within 'Option' is invalid.
    let o = &option;
    for c in o.chars() {
        if c.is_whitespace() {
            option = "InvalidOption".to_string();
            return (option, "".to_string(), vec![]);
        }
    }

    i = value.find(attr_delimit_char);
    let primary_value;
    let mut tmp_attr_vec: Vec<&str> = Vec::new();
    let attributes;
    match i.is_some() {
        true => {
            primary_value = format!("{}", &value[..i.unwrap()].trim());
            attributes = format!("{}", &value[i.unwrap() + 1..]);
            tmp_attr_vec = attributes.split(attr_delimit_char).collect();
        }
        false => primary_value = format!("{}", value.to_string()),
    }

    let mut attr_vec: Vec<String> = Vec::new();
    for a in &tmp_attr_vec {
        attr_vec.push(a.trim().to_string());
    }

    (option, primary_value, attr_vec)
}

#[test]
fn test_parse_file() {
    let line1 = OptionProperties {
        option: "option".to_string(),
        value: Value {
            primary: "Blue".to_string(),
            attributes: vec!["light".to_string(), "shiny".to_string()],
        },
    };

    let line2 = OptionProperties {
        option: "max_users".to_string(),
        value: Value {
            primary: "30".to_string(),
            attributes: vec![],
        },
    };

    let line3 = OptionProperties {
        option: "DelayOff".to_string(),
        value: Value {
            primary: "".to_string(),
            attributes: vec![],
        },
    };

    assert_eq!(
        parse_file("./config_test.conf", ',').unwrap(),
        vec![line1, line2, line3]
    );
}

#[test]
fn test_parse_line() {
    // Test with no attributes
    assert_eq!(
        parse_line("Option = /home/foo", ','),
        ("Option".to_string(), "/home/foo".to_string(), vec![])
    );

    // Test with 5 attributes and several spaces
    assert_eq!(
        parse_line("Option=/home/foo , another  ,   test,1,2,3", ','),
        (
            "Option".to_string(),
            "/home/foo".to_string(),
            vec![
                "another".to_string(),
                "test".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string()
            ]
        )
    );

    // Test with leading '#' sign
    assert_eq!(
        parse_line("#Option = /home/foo", ','),
        ("".to_string(), "".to_string(), vec![])
    );

    // Test with two attributes, a single space after the commas
    assert_eq!(
        parse_line("Option = /home/foo, removable, test", ','),
        (
            "Option".to_string(),
            "/home/foo".to_string(),
            vec!["removable".to_string(), "test".to_string()]
        )
    );

    // Test for blank line
    assert_eq!(
        parse_line("        ", ','),
        ("".to_string(), "".to_string(), vec![])
    );

    // Test for whitespace in Option
    assert_eq!(
        parse_line("Option  /home/foo", ','),
        ("InvalidOption".to_string(), "".to_string(), vec![])
    );

    // Test for '=' after Option has already been marked as invalid.
    assert_eq!(
        parse_line("Option  /home/foo = value", ','),
        ("InvalidOption".to_string(), "".to_string(), vec![])
    );
}
