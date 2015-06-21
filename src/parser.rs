use std::io::BufReader;

use vcard::Property;
use std::str::FromStr;

struct Parser;

impl Parser {
    fn new() -> Parser {
        Parser
    }

    pub fn parse_vcard(&self, input: String) {
        let lines: Vec<String> = input.split("\n").map(|l| l.to_string()).collect();
        let properties: Vec<Result<Property,_>> = merge_lines(lines).iter().map(|line| to_property(line)).collect();
    }

}

#[derive(Debug)]
pub struct ParserError {
    msg: String
}


impl ParserError {
    pub fn new(msg: String) -> ParserError {
        ParserError { msg: msg }
    }
}


/// A property can be split over multiple lines, e.g.
/// 
/// ```text
/// ADR;TYPE=home;LABEL="Heidestraße 17\n51147 Köln\nDeutschland"
///  :;;Heidestraße 17;Köln;;51147;Germany
/// ```
/// 
/// Note the additional space at the second line.
/// This method takes a vector of lines and merges every line with the following lines.
fn merge_lines(lines: Vec<String>) -> Vec<String> {
    match lines.iter().position(|line| line.starts_with(" ")) {
        Some(index) => {
            let fused_lines = merge_lines_at(lines, index);
            return merge_lines(fused_lines);
        },
        None => { return lines; }
    }
}

/// This method joins the contents (index-1, index) together
fn merge_lines_at(mut lines: Vec<String>, index: usize) -> Vec<String> {
    // if the first line was already prefixed with a space, something was wrong before
    assert!(index > 0);

    let new_line = lines[index-1].clone() + &lines[index];
    lines[index-1] = new_line;
    lines.remove(index);

    lines
}


/// Transforms one string representation of a property to a internal one
fn to_property(line: &str) -> Result<Property,ParserError> {
    // The RFC defines a contentline as:
    //
    //     contentline = [group "."] name *(";" param) ":" value CRLF
    let property_regex = regex!(r"^(?P<group>[a-zA-Z0-9-]+\.)?(?P<name>[a-zA-Z0-9-]+)(?P<params>;.*)?:(?P<value>.*)$");


    let property;

    match property_regex.captures(line) {
        Some(c) => {
            // group and params are optional
            let group = c.name("group");
            let params = c.name("params");

            // name and value have to be there
            let name;
            match c.name("name") {
                Some(v) => { name = v },
                None => {
                    return Err(ParserError::new(format!("line withouth name: {:?}", line)));
                }
            }
            let value;
            match c.name("value") {
                Some(v) => { value = v },
                None => {
                    return Err(ParserError::new(format!("line withouth value: {:?}", line)));
                }
            }

            property = Property::new_from_strings(group, name, params, value);
        }

        None => {
            return Err(ParserError::new(format!("line withouth match: {:?}", line)));
        }
    }

    println!("{:?}", property);
    match property {
        Ok(p) => Ok(p),
        Err(_) => Err(ParserError::new("random erruor".to_string()))
    }
}



#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;

    use parser::Parser;

    #[test]
    fn wikipedia_example() {
        let path = Path::new("example-vcards/wikipedia_example.vcf");
        let mut reader = BufReader::new(File::open(&path).unwrap());
        let mut result = String::new();
        reader.read_to_string(&mut result);

        let parser = Parser::new();
        parser.parse_vcard(result);        
    }
}