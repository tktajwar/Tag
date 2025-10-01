use std::fmt;
use std::fmt::Display;
use regex::Regex;
use linked_hash_map::LinkedHashMap;

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub struct TagID {
    exponent: usize,
    mantissa: String,
}

impl From<&str> for TagID {

    /// Creates TagID from string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagID::from("@1.0");
    /// ```

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
    /// Generates a TagID that comes after a TagID.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagID::from("@1.0");
    /// let b = tag::TagID::generate_next(&a);
    ///
    /// assert_eq!(b, tag::TagID::from("@2.0"));
    /// assert!(a < b);
    /// ```

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

    /// Generates a TagID between two TagIDs.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagID::from("@1.0");
    /// let d = tag::TagID::from("@2.0");
    /// let b = tag::TagID::generate_between(&a, &d);
    /// let c = tag::TagID::generate_between(&b, &d);
    ///
    /// assert!(a < b && b < c && c < d);
    /// ```

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
    Invalid(&'a str),
    ID(&'a str),
    Title(&'a str),
    Flags(&'a str),
    Attribute(&'a str),
}

impl <'a>TagField<'a> {
    fn from(field_str: &'a str) -> TagField<'a> {
	if field_str.len() == 0 {
	    return TagField::Invalid(field_str)
	}

	return match field_str.as_bytes()[0] {
	    b'@' => {
		let re = Regex::new(r"^@[0-9a-zA-Z]+\.[0-9a-zA-Z]*$").unwrap();
		if re.is_match(field_str) {
		    TagField::ID(field_str)
		} else {
		    TagField::Invalid(field_str)
		}
	    },
	    b'#' => {
		let re = Regex::new(r"^#[0-9a-zA-Z_\-]+(\s*#[0-9a-zA-Z_\-]*)*$").unwrap();
		if re.is_match(field_str) {
		    TagField::Flags(field_str)
		} else {
		    TagField::Invalid(field_str)
		}
	    },
	    b':' => {
		let re = Regex::new(r"^:[0-9a-zA-Z_\-\s]+:[0-9a-zA-Z_\-\.\,\s]*$").unwrap();
		if re.is_match(field_str) {
		    TagField::Attribute(field_str)
		} else {
		    TagField::Invalid(field_str)
		}
	    },
	    _   => TagField::Title(field_str),
	}
    }

    /// Returns a vector of flags of the field, or `None` if the field
    /// isn't flags type.
    ///
    /// # Examples
    ///
    /// ```
    /// let f1 = tag::TagField::Flags("#hello #world");
    ///
    /// assert_eq!(f1.flags(), Some(vec![
    ///     "#hello".to_string(),
    ///     "#world".to_string(),
    /// ]));
    ///
    /// let f2 = tag::TagField::Attribute(":atr: value");
    ///
    /// assert_eq!(f2.flags(), None);
    /// ```

    pub fn flags(&self) -> Option<Vec<String>> {
	let re = Regex::new(r"#[0-9a-zA-Z_\-]+").unwrap();
	match self {
	    TagField::Flags(field_str) => Some(re.find_iter(field_str)
					       .map(|m| m.as_str().to_string())
					       .collect()),
	    _ => None,
	}
    }

    /// Returns String of flags after concatenating with the field's
    /// flags, or `None` if `flags` argument is invalid or if the
    /// field isn't of flag type.
    ///
    /// # Examples
    ///
    /// ```
    /// let f1 = tag::TagField::Flags("#hello #world");
    /// assert_eq!(f1.concat_flags("#rust #program"),
    ///            Some("#hello #world #rust #program".to_string()));
    /// assert_eq!(f1.concat_flags(":attr: value"), None);
    /// ```
    ///
    /// ```
    /// let f2 = tag::TagField::Title("Not a TagField::Flags");
    /// assert_eq!(f2.concat_flags("#rust #program"), None);
    /// ```

    pub fn concat_flags(&self, flags: &str) -> Option<String> {
	let re = Regex::new(r"^#[0-9a-zA-Z_\-]+(\s#[0-9a-zA-Z_\-]+)*$").unwrap();

	let Some(flags) = re.find(flags) else {
	    return None;
	};
	let con_flags = flags.as_str();

	match self {
	    TagField::Flags(field_str) => Some(field_str.to_string() + " " + con_flags),
	    _ => None,
	}
    }

    /// Returns String of flags after removing the flags from the
    /// field, or `None` if if the field isn't of flag type.
    ///
    /// # Examples
    ///
    /// ```
    /// let f1 = tag::TagField::Flags("#hello #world");
    /// assert_eq!(f1.sincat_flags("#world"),
    ///            Some("#hello".to_string()));
    /// assert_eq!(f1.sincat_flags(":attr: value"),
    ///            Some("#hello #world".to_string()));
    /// ```
    ///
    /// ```
    /// let f2 = tag::TagField::Title("Not a TagField::Flags");
    /// assert_eq!(f2.sincat_flags("#hello #world"), None);
    /// ```

    pub fn sincat_flags(&self, flags: &str) -> Option<String> {
	let re = Regex::new(r"(#[0-9a-zA-Z_\-]+)").unwrap();
	let sin_flags: Vec<String> = re.find_iter(&flags)
	    .map(|m| m.as_str().to_string())
	    .collect();

	match self {
	    TagField::Flags(_) => (),
	    _ => return None,
	};

	let mut flags_sinned = String::new();
	let Some(old_flags) = self.flags() else {
	    return None;
	};
	for flag in old_flags {
	    if !sin_flags.contains(&flag) {
		flags_sinned.push_str(&(flag + " "));
	    }
	}

	Some(flags_sinned.trim().to_string())
    }

    /// Returns the attribute key (String), or `None` if it's not an
    /// attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// let f1 = tag::TagField::Attribute(":src: code");
    /// assert_eq!(f1.attribute_key(), Some(":src:".to_string()));
    /// ```
    ///
    /// ```
    /// let f2 = tag::TagField::Title("hello world");
    /// assert_eq!(f2.attribute_key(), None);
    /// ```

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

    /// Returns the attribute value (String), or `None` if it's not an
    /// attribute or if the value is void.
    ///
    /// # Examples
    ///
    /// ```
    /// let f1 = tag::TagField::Attribute(":src: code");
    /// assert_eq!(f1.attribute_value(), Some("code".to_string()));
    /// ```
    ///
    /// ```
    /// let f2 = tag::TagField::Title("hello world");
    /// assert_eq!(f2.attribute_value(), None);
    /// ```
    ///
    /// ```
    /// let f3 = tag::TagField::Attribute(":existentialism:");
    /// assert_eq!(f3.attribute_value(), None);
    /// ```

    pub fn attribute_value(&self) -> Option<String> {
	let re = Regex::new(r"^:[0-9a-zA-Z_\-]+:\s*([0-9a-zA-Z_\-\.\,\s]+$)").unwrap();
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

    /// Returns both attribute key and value, or `None` if it's not an
    /// attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// let f1 = tag::TagField::Attribute(":src: code");
    /// assert_eq!(f1.attribute_key_value(), Some((
    ///     Some(":src:".to_string()),
    ///     Some("code".to_string()),
    /// )));
    /// ```
    ///
    /// ```
    /// let f2 = tag::TagField::Title("hello world");
    /// assert_eq!(f2.attribute_key_value(), None);
    /// ```
    ///
    /// ```
    /// let f3 = tag::TagField::Attribute(":existentialism:");
    /// assert_eq!(f3.attribute_key_value(), Some((
    ///     Some(":existentialism:".to_string()),
    ///     None,
    /// )));
    /// ```

    pub fn attribute_key_value(&self) -> Option<(Option<String>,Option<String>)> {
	let re = Regex::new(r"^(:[0-9a-zA-Z_\-]+:)\s*([0-9a-zA-Z_\-\.\,\s]+$)?").unwrap();
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

    /// Returns a vector of fields of given TagItem.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | My Title | #hello #world | :src: code |\
    /// :invalid | #valid-flag | #inva!!lid");
    /// let fields = a.fields();
    /// assert_eq!(fields[0], tag::TagField::ID("@1.0"));
    /// assert_eq!(fields[1], tag::TagField::Title("My Title"));
    /// assert_eq!(fields[2], tag::TagField::Flags("#hello #world"));
    /// assert_eq!(fields[3], tag::TagField::Attribute(":src: code"));
    /// assert_eq!(fields[4], tag::TagField::Invalid(":invalid"));
    /// assert_eq!(fields[5], tag::TagField::Flags("#valid-flag"));
    /// assert_eq!(fields[6], tag::TagField::Invalid("#inva!!lid"));
    /// ```

    pub fn fields(&self) -> Vec<TagField> {
	let mut fields = Vec::new();

	let field_split = self.tag_line.split("|").map(|x| x.trim());

	for field_str in field_split {
	    let field = TagField::from(field_str);
	    fields.push(field);
	}

	fields
    }

    /// Returns a vector of flags of the given TagItem.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | My Title | #hello #world |\
    ///                        :invalid | #valid-flag | #inva!!lid");
    /// assert_eq!(a.flags(), Some(vec![
    ///     "#hello".to_string(),
    ///     "#world".to_string(),
    ///     "#valid-flag".to_string(),
    /// ]));
    /// ```
    ///
    /// ```
    /// let b = tag::TagItem::from("@1.0 | Item with no flags");
    /// assert_eq!(b.flags(), None);
    /// ```

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

    /// Returns `true` if the item has the given flags.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | My Title | #hello #world |\
    ///                             :invalid | #valid-flag | #inva!!lid");
    ///
    /// assert!(!(a.has_flag("#test".to_string())));
    ///
    /// let b = tag::TagItem::from("@1.0 | Item with no flags");
    ///
    /// assert!(!(b.has_flag("#hello".to_string())));
    /// ```

    pub fn has_flag(&self, flag: String) -> bool {
	if let Some(flags) = self.flags() {
	    flags.contains(&flag)
	} else {
	    false
	}
    }

    /// Returns `true` if the item has all the given flag.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | My Title | #hello #world |\
    ///                        :invalid | #valid-flag | #inva!!lid");
    /// assert!(a.has_flags(vec![
    ///     "#hello".to_string(),
    ///     "#world".to_string(),
    ///     "#valid-flag".to_string(),
    /// ]));
    /// assert!(!a.has_flags(vec![
    ///     "#does".to_string(),
    ///     "#not".to_string(),
    ///     "#have".to_string(),
    /// ]));
    /// ```

    pub fn has_flags(&self, flags: Vec<String>) -> bool {
	for flag in flags {
	    if !(self.has_flag(flag)) {
		return false;
	    }
	}
	true
    }

    /// Concatenate flags to the `field_no`th field if it's of
    /// type flags.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from(
    ///     "@1.0 | Field Indexing is Zero-based | #hello #world"
    /// );
    /// a.add_flags_to_field_no("#new #flags", 2);
    ///
    /// assert_eq!(a.flags(), Some(vec![
    ///     "#hello".to_string(),
    ///     "#world".to_string(),
    ///     "#new".to_string(),
    ///     "#flags".to_string(),
    /// ]));
    /// ```

    pub fn add_flags_to_field_no(&mut self, flags: &str, field_no: usize) {
	let fields = self.fields();
	let Some(old_field) = fields.get(field_no) else {return};
	let Some(new_field) = old_field.concat_flags(flags) else {return};

	let mut new_tagline = String::new();
	for field in &fields[ ..field_no] {
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	    new_tagline.push_str(" | ");
	}
	new_tagline.push_str(&new_field);
	for field in &fields[field_no+1.. ] {
	    new_tagline.push_str(" | ");
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	}
	self.tag_line = new_tagline;
    }

    /// Remove flags from the `field_no`th field if it's of type
    /// flags.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from(
    ///     "@1.0 | Field Indexing is Zero-based | #hello #world #new #flags"
    /// );
    /// a.remove_flags_from_field_no("#hello #new", 2);
    ///
    /// assert_eq!(a.flags(), Some(vec![
    ///     "#world".to_string(),
    ///     "#flags".to_string(),
    /// ]));
    /// ```

    pub fn remove_flags_from_field_no(&mut self, flags: &str, field_no: usize) {
	let fields = self.fields();
	let Some(old_field) = fields.get(field_no) else {return};
	let Some(new_field) = old_field.sincat_flags(flags) else {return};

	let mut new_tagline = String::new();
	for field in &fields[ ..field_no] {
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	    new_tagline.push_str(" | ");
	}

	if new_field.len() > 0 {
	    new_tagline.push_str(&new_field);
	} else {
	    if new_tagline.len() >= 3 {
		new_tagline.pop();
		new_tagline.pop();
		new_tagline.pop();
	    }
	}

	for field in &fields[field_no+1.. ] {
	    new_tagline.push_str(" | ");
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	}

	self.tag_line = new_tagline;
    }

    /// Add flags to the item.
    ///
    /// If the item doesn't have any flags field, a new one will
    /// appended, otherwise it'll use the last Flag field.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from(
    ///     "@1.0 | Already with Flags | #hello #world"
    /// );
    /// a.add_flags("#new #flags");
    ///
    /// assert_eq!(a.flags(), Some(vec![
    ///     "#hello".to_string(),
    ///     "#world".to_string(),
    ///     "#new".to_string(),
    ///     "#flags".to_string(),
    /// ]));
    /// ```
    ///
    /// ```
    /// let mut b = tag::TagItem::from(
    ///     "@1.0 | No Prior Flags"
    /// );
    /// b.add_flags("#you-have-a-flag-now");
    ///
    /// assert_eq!(b.flags(), Some(vec![
    ///     "#you-have-a-flag-now".to_string(),
    /// ]));
    /// ```

    pub fn add_flags(&mut self, flags: &str) {
	let fields = self.fields();
	for field_no in (0..fields.len()).rev() {
	    if let TagField::Flags(_) = fields[field_no] {
		self.add_flags_to_field_no(flags, field_no);
		return;
	    }
	}
	self.tag_line.push_str(" | ");
	self.tag_line.push_str(&flags);
    }

    /// Remove flags from the item.
    ///
    /// The removal is done to every flag fields of the item.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from(
    ///     "@1.0 | Flags | #hello #world"
    /// );
    /// a.remove_flags("#hello");
    ///
    /// assert_eq!(a.flags(), Some(vec![
    ///     "#world".to_string(),
    /// ]));
    /// ```
    ///
    /// ```
    /// let mut b = tag::TagItem::from(
    ///     "@1.0 | #many | #many #flags | #happy #flags | #cool"
    /// );
    /// b.remove_flags("#many #happy");
    ///
    /// assert_eq!(b.flags(), Some(vec![
    ///     "#flags".to_string(),
    ///     "#flags".to_string(),
    ///     "#cool".to_string(),
    /// ]));
    /// ```

    pub fn remove_flags(&mut self, flags: &str) {
	let mut new_tagline = "".to_string();
	for field in self.fields() {
	    let s = match field {
		TagField::Flags(_) => &field.sincat_flags(flags).unwrap(),
		TagField::ID(s) |
		TagField::Title(s) |
		TagField::Attribute(s) |
		TagField::Invalid(s) => s,
	    };

	    if s.len() > 0 {
		new_tagline.push_str(s);
		new_tagline.push_str(" | ");
	    }
	}

	'clean_end: loop {
	    let Some(c) = new_tagline.pop() else {break 'clean_end};

	    if c == ' ' || c == '|' {
		new_tagline.push(c);
		break 'clean_end;
	    }
	}

	self.tag_line = new_tagline;
    }

    /// Returns a vector of all the attributes.
    ///
    /// An attribute is represented with `(Key, value): (<Option<String>, Option<String>>)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | :src: code | :null:");
    ///
    /// assert_eq!(a.attributes(), vec![
    ///     (Some(":src:".to_string()), Some("code".to_string())),
    ///     (Some(":null:".to_string()), None),
    /// ]);
    /// ```
    ///
    /// ```
    /// let b = tag::TagItem::from("@1.0 | Item with no attributes");
    ///
    /// assert_eq!(b.attributes(), vec![]);
    /// ```

    pub fn attributes(&self) -> Vec<(Option<String>,Option<String>)> {
	let mut attributes: Vec<(Option<String>, Option<String>)> = Vec::new();

	for field in self.fields() {
	    match field.attribute_key_value() {
		Some(attribute_key_value) => attributes.push(attribute_key_value),
		None => (),
	    }
	}

	attributes
    }

    /// Returns the attribute with the given key, or `None` if it's
    /// not available.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | :src: code");
    /// assert_eq!(a.fetch_attribute(":src:".to_string()), Some((
    ///     Some(":src:".to_string()),
    ///     Some("code".to_string()),
    /// )));
    /// assert_eq!(a.fetch_attribute(":ABCD:".to_string()), None);
    /// ```

    pub fn fetch_attribute(&self, key: String) -> Option<(Option<String>,Option<String>)> {
	for attribute in self.attributes() {
	    if let Some(ref attribute_key) = attribute.0 {
		if *attribute_key == key {
		    return Some(attribute)
		}
	    }
	}
	None
    }

    /// Returns `true` if the item has given attribute key.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | :src: code");
    /// assert!(a.has_attribute(":src:".to_string()));
    /// assert!(!a.has_attribute(":null:".to_string()));
    /// ```

    pub fn has_attribute(&self, key: String) -> bool {
	self.fetch_attribute(key) != None
    }

    /// Returns `true` if the item matches the given attribute.
    ///
    /// If the item has multiple attributes with the same key, it will
    /// only check the first one.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | :src: code | :src: new");
    /// assert!(a.match_attribute((
    ///     Some(":src:".to_string()),
    ///     Some("code".to_string()),
    /// )));
    /// assert!(!a.match_attribute((
    ///     Some(":attr:".to_string()),
    ///     Some("doesn't have".to_string()),
    /// )));
    /// assert!(!a.match_attribute((
    ///     Some(":src:".to_string()),
    ///     Some("new".to_string()),
    /// )));
    /// ```

    pub fn match_attribute(&self, attribute: (Option<String>, Option<String>)) -> bool {
	let Some(ref key) = attribute.0 else {
	    return false;
	};
	let Some(attribute_to_fetch) = self.fetch_attribute(key.to_string()) else {
	    return false;
	};
	attribute_to_fetch == attribute
    }

    /// Sets field's attribute key and value if it's an attribute
    /// field.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from("@1.0 | Program source | :src: code");
    /// a.set_attribute_at_field_no((Some(":src:"), Some("tag.rs")), 2);
    ///
    /// assert!(a.match_attribute((
    ///     Some(":src:".to_string()),
    ///     Some("tag.rs".to_string()),
    /// )));
    /// ```

    pub fn set_attribute_at_field_no(&mut self, attribute: (Option<&str>, Option<&str>), field_no: usize) {
	let fields = self.fields();
	let TagField::Attribute(_) = fields[field_no] else { return };
	let mut new_tagline = String::new();

	for field in &fields[ ..field_no] {
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	    new_tagline.push_str(" | ");
	}

	if let Some(attribute_key) = attribute.0 {
	    if attribute_key.as_bytes()[0] != b':' { return };
	    if attribute_key.as_bytes()[attribute_key.len()-1] != b':' { return };
	    new_tagline.push_str(attribute_key);
	} else { return }

	if let Some(attribute_val) = attribute.1 {
	    new_tagline.push_str(" ");
	    new_tagline.push_str(attribute_val);
	}

	for field in &fields[field_no+1.. ] {
	    new_tagline.push_str(" | ");
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	}

	self.tag_line = new_tagline;
    }

    /// Sets the attribute with the given key.
    ///
    /// It'll first look if there's already an attribute with the
    /// given key. If no attribute is found, it'll insert a new one.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from("@1.0 | Program source | :src: code");
    /// a.set_attribute((Some(":src:"), Some("tag.rs")));
    /// a.set_attribute((Some(":attr:"), Some("value")));
    ///
    /// assert!(a.match_attribute((
    ///     Some(":src:".to_string()),
    ///     Some("tag.rs".to_string()),
    /// )));
    /// assert!(a.match_attribute((
    ///     Some(":attr:".to_string()),
    ///     Some("value".to_string()),
    /// )));
    /// ```

    pub fn set_attribute(&mut self, attribute: (Option<&str>, Option<&str>)) {
	let Some(attribute_key) = attribute.0 else { return };
	if attribute_key.as_bytes()[0] != b':' { return };
	if attribute_key.as_bytes()[attribute_key.len()-1] != b':' { return };

	for field_no in 0..self.fields().len() {
	    let field = &self.fields()[field_no];
	    if field.attribute_key() == Some(attribute_key.to_string()) {
		self.set_attribute_at_field_no(attribute, field_no);
		return;
	    }
	}

	self.tag_line.push_str(" | ");
	self.tag_line.push_str(attribute_key);
	if let Some(attribute_val) = attribute.1 {
	    self.tag_line.push_str(" ");
	    self.tag_line.push_str(attribute_val);
	}
    }

    /// Remove the attribute with the given key.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = tag::TagItem::from("@1.0 | Program source | :src: code");
    /// a.remove_attribute(":src:");
    /// a.remove_attribute(":does-not-have-this-one-but-ok");
    ///
    /// assert!(!a.has_attribute(":src:".to_string()));
    /// ```

    pub fn remove_attribute(&mut self, attribute_key: &str) {
	let mut new_tagline = String::new();
	let fields = &self.fields();

	{
	    let field = &fields[0];
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	}

	for field in &fields[1.. ] {
	    if field.attribute_key() == Some(attribute_key.to_string()) { continue }
	    new_tagline.push_str(" | ");
	    match field {
		TagField::ID(field_str) |
		TagField::Title(field_str) |
		TagField::Flags(field_str) |
		TagField::Attribute(field_str) |
		TagField::Invalid(field_str)  => new_tagline.push_str(field_str),
	    }
	}

	self.tag_line = new_tagline;
    }
}

impl From<&str> for TagItem {

    /// Creates TagItem from string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = tag::TagItem::from("@1.0 | My Title | #hello #world");
    /// ```

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
