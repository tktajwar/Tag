use std::fmt;
use regex::Regex;

#[derive(PartialEq, PartialOrd)]
pub struct TagID {
    exponent: usize,
    mantissa: String,
}

impl From<&str> for TagID {
    fn from(tag_number: &str) -> TagID {
	let mut mantissa: String = String::with_capacity(tag_number.len() - 1);
	let mut start: usize = 1;

	// ignore leading zeros
	while start < tag_number.len() && tag_number.as_bytes()[start] == b'0' {
	    start += 1;
	}
	let mut i = start;

	// parse until radix point
	 while i < tag_number.len() {
	     match tag_number.as_bytes()[i] {
		 b'0'..=b'9'
		     | b'A'..=b'Z'
		     | b'a'..=b'z'
		     => mantissa.push(tag_number.as_bytes()[i] as char),
		 _ => break,
	     }
	     i += 1;
	}
	let exponent: usize = i - start;

	// parse the rest of the string
	 while i < tag_number.len() {
	     match tag_number.as_bytes()[i] {
		 b'0'..=b'9'
		     | b'A'..=b'Z'
		     | b'a'..=b'z'
		     => mantissa.push(tag_number.as_bytes()[i] as char),
		 b'.' => (),
		 _ => break,
	     }
	     i += 1;
	 }

	// remove trailing zeros
	while mantissa.len() > exponent {
	    if mantissa.as_bytes()[mantissa.len()-1] == b'0' {
		mantissa.pop();
	    }
	    else {
		break
	    }
	}

	// avoid @0.0
	if mantissa.len() == 0 {
	    panic!("Tried to create non-positive TagID!");
	}

	TagID { exponent, mantissa }
    }
}

impl fmt::Display for TagID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let left = {
	    if self.exponent > 0 {
		&self.mantissa[0..self.exponent]
	    }
	    else {
		"0"
	    }
	};
	let right = {
	    if self.mantissa.len() > self.exponent {
		&self.mantissa[self.exponent..self.mantissa.len()]
	    }
	    else {
		"0"
	    }
	};
        write!(f, "@{left}.{right}")
    }
}

impl TagID {
    pub fn generate_next(tag_id: &TagID) -> TagID {
	let mut exponent = tag_id.exponent;
	let mut mantissa = tag_id.mantissa.clone();

	let Some(mut last_digit) = mantissa.pop() else {
	    panic!("Coudln't get the last digit");
	};

	while last_digit == 'z' {
	    if mantissa.len() == 0 {
		mantissa.push('1');
		for _ in 0..exponent {
		    mantissa.push('0');
		}
		exponent = exponent + 1;
		return TagID{ exponent, mantissa };
	    }
	    match mantissa.pop() {
		Some(c) => last_digit = c,
		None => panic!("Couldn't get the suitable digit"),
	    }
	}

	last_digit = match last_digit {
	    '0'..'9'
		| 'A'..'Z'
		| 'a'..'z'
		=> (last_digit as u8 + 1 as u8) as char,
	    '9' => 'A',
	    'Z' => 'a',
	    _ => panic!("Invalid last digit!"),
	};

	mantissa.push(last_digit);

	while mantissa.len() < exponent {
	    mantissa.push('0');
	}

	TagID{ exponent, mantissa }
    }

    pub fn generate_between(smaller_id: &TagID, larger_id: &TagID) -> TagID {
	if !(smaller_id < larger_id) {
	    panic!("Smaller ID must be smaller than the Larger ID!");
	}

	let middle_id = TagID::generate_next(smaller_id);

	if middle_id < *larger_id {
	    return middle_id;
	}

	let exponent = smaller_id.exponent;
	let mantissa = smaller_id.mantissa.clone() + "1";

	let mut middle_id = TagID{ exponent, mantissa };

	while middle_id >= *larger_id {
	    if middle_id.mantissa.len() > middle_id.exponent {
		middle_id.mantissa.pop();
	    }
	    middle_id.mantissa.push_str("01");
	}

	middle_id
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TagField<'a> {
    Invalid,
    ID(&'a str),
    Title(&'a str),
    Flags(&'a str),
    Attribute(&'a str),
}

impl <'a>TagField<'a> {
    fn from (field_str: &'a str) -> TagField<'a> {
	if field_str.len() == 0 {
	    return TagField::Invalid
	}

	return match field_str.as_bytes()[0] {
	    b'@' => {
		let re = Regex::new(r"^@[0-9a-zA-Z]+\.[0-9a-zA-Z]*$").unwrap();
		if re.is_match(field_str) {
		    TagField::ID(field_str)
		} else {
		    TagField::Invalid
		}
	    },
	    b'#' => {
		let re = Regex::new(r"^#[0-9a-zA-Z_\-]+(\s*#[0-9a-zA-Z_\-]*)*$").unwrap();
		if re.is_match(field_str) {
		    TagField::Flags(field_str)
		} else {
		    TagField::Invalid
		}
	    },
	    b':' => {
		let re = Regex::new(r"^:[0-9a-zA-Z_\-\s]+:[0-9a-zA-Z_\-\s]*$").unwrap();
		if re.is_match(field_str) {
		    TagField::Attribute(field_str)
		} else {
		    TagField::Invalid
		}
	    },
	    _   => TagField::Title(field_str),
	}
    }

    pub fn flags (&self) -> Option<Vec<String>> {
	let re = Regex::new(r"#[0-9a-zA-Z_\-]+").unwrap();
	match self {
	    TagField::Flags(field_str) => Some(re.find_iter(field_str)
					       .map(|m| m.as_str().to_string())
					       .collect()),
	    _ => None,
	}
    }

    pub fn attribute_key(&self) -> Option<String> {
	let re = Regex::new(r"^:[0-9a-zA-Z_\-\s]+:").unwrap();
	match self {
	    TagField::Attribute(field_str) => {
		if let Some(key) = re.find(field_str) {
		    Some(key.as_str().to_string())
		} else {
		    None
		}
	    },
	    _ => None,
	}
    }

    pub fn attribute_value(&self) -> Option<String> {
	let re = Regex::new(r"^:[0-9a-zA-Z_\-]+:\s*([0-9a-zA-Z_\-\s]+$)").unwrap();
	match self {
	    TagField::Attribute(field_str) => {
		let Some(captures) = re.captures(field_str) else {
		    return None
		};
		if let Some(value) = captures.get(1) {
		    Some(value.as_str().to_string())
		} else {
		    None
		}
	    },
	    _ => None,
	}
    }

    pub fn attribute_key_value(&self) -> Option<(Option<String>,Option<String>)> {
	let re = Regex::new(r"^(:[0-9a-zA-Z_\-]+:)\s*([0-9a-zA-Z_\-\s]+$)?").unwrap();
	match self {
	    TagField::Attribute(field_str) => {
		let Some(captures) = re.captures(field_str) else {
		    return None
		};
		Some((
		    if let Some(key) = captures.get(1) {
			Some(key.as_str().to_string())
		    } else {
			None
		    },
		    if let Some(value) = captures.get(2) {
			Some(value.as_str().to_string())
		    } else {
			None
		    },
		))
	    },
	    _ => None,
	}
    }
}

pub struct TagItem {
    id: TagID,
    tag_line: String,
}

impl TagItem {
    pub fn fields(&self) -> Vec<TagField> {
	let mut fields = Vec::new();

	let field_split = self.tag_line.split("|").map(|x| x.trim());

	for field_str in field_split {
	    let field = TagField::from(field_str);
	    fields.push(field);
	}

	fields
    }

    pub fn flags(&self) -> Option<Vec<String>> {
	let mut flags: Vec<String> = Vec::new();

	for field in self.fields() {
	    match field.flags() {
		Some(new_flags) => flags.extend(new_flags.to_owned()),
		None => (),
	    }
	}

	if flags.len() != 0 {
	    Some(flags)
	} else {
	    None
	}
    }

    pub fn has_flag(&self, flag: String) -> bool {
	if let Some(flags) = self.flags() {
	    flags.contains(&flag)
	} else {
	    false
	}
    }

    pub fn has_flags(&self, flags: Vec<String>) -> bool {
	for flag in flags {
	    if !(self.has_flag(flag)) {
		return false;
	    }
	}
	true
    }

    pub fn attributes(&self) -> Option<Vec<(Option<String>,Option<String>)>> {
	let mut attributes: Vec<(Option<String>, Option<String>)> = Vec::new();

	for field in self.fields() {
	    match field.attribute_key_value() {
		Some(attribute_key_value) => attributes.push(attribute_key_value),
		None => (),
	    }
	}

	if attributes.len() != 0 {
	    Some(attributes)
	} else {
	    None
	}
    }

    pub fn fetch_attribute(&self, key: String) -> Option<(Option<String>,Option<String>)> {
	if let Some(attributes) = self.attributes() {
	    for attribute in attributes {
		if let Some(ref attribute_key) = attribute.0 {
		    if *attribute_key == key {
			return Some(attribute)
		    }
		}
	    }
	    None
	} else {
	    None
	}
    }

    pub fn has_attribute(&self, key: String) -> bool {
	self.fetch_attribute(key) != None
    }
}

impl From<&str> for TagItem {
    fn from(tag_line: &str) -> TagItem {
	let mut pipe_end = 0;

	while pipe_end < tag_line.len() {
	    if tag_line.as_bytes()[pipe_end] == b'|' {
		break;
	    }
	    pipe_end += 1;
	}

	let id = TagID::from(&tag_line[0..pipe_end]);

	let tag_line = String::from(tag_line);

	TagItem { id, tag_line }
    }
}

impl fmt::Display for TagItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{}", self.tag_line)
    }
}
