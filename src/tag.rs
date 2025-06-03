use std::fmt;

#[derive(PartialEq, PartialOrd)]
pub struct TagID {
    exponent: usize,
    mantissa: String,
}

impl From<&String> for TagID {
    fn from(tag_number: &String) -> TagID {
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

	TagID { exponent, mantissa }
    }
}

impl From<&str> for TagID {
    fn from(tag_number: &str) -> TagID {
	TagID::from(&String::from(tag_number))
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
