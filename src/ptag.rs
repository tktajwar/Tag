use crate::TagItem;
use crate::TagID;

use regex::Regex;
use std::sync::LazyLock;
use std::fs::File;
use memmap2::Mmap;
use std::error::Error;
use std::cmp::max;

static RE_TAG_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(
	r"^\s*@([0-9]+\.?[0-9]*).*$"
    ).unwrap());

pub struct PlainTag {
    tagfile: Mmap,
    list_start: usize,
}

impl PlainTag {

    /// Returns an iterator over the tag items of the tag file.
    ///
    /// # Examples
    ///
    /// ```
    /// let ptag = tag::PlainTag::try_from("Tagfile").unwrap();
    /// let mut tag_items = ptag.items();
    /// ```

    pub fn items(&self) -> PTagIterator {
	PTagIterator {
	    iter: str::from_utf8(
		&self.tagfile[self.list_start..])
		.unwrap()
		.split('\n')
	}
    }

    /// Returns the tag item with the given tag ID after performing a
    /// linear search, or `None` if it's not found.
    ///
    /// # Examples
    ///
    /// ```
    /// let ptag = tag::PlainTag::try_from("Tagfile").unwrap();
    /// let id = tag::TagID::try_from("@1.0").unwrap();
    /// let a = ptag.linear_search(id);
    /// ```

    pub fn linear_search(&self, id: TagID) -> Option<TagItem> {
	for item in self.items() {
	    let Ok(item) = item else {continue};
	    if item.id == id {
		return Some(item);
	    }
	}
	None
    }

    /// Returns the tag item with the given tag ID after performing a
    /// binary search, or Error on ID parse failure or if the item
    /// isn't found.
    ///
    /// # Examples
    ///
    /// ```
    /// let ptag = tag::PlainTag::try_from("Tagfile").unwrap();
    /// let id = tag::TagID::try_from("@1.0").unwrap();
    /// let a = ptag.binary_search(id);
    /// assert!(a.is_ok());
    /// ```

    pub fn binary_search(
	&self,
	id: TagID
    ) -> Result<TagItem, Box<dyn Error>> {
	let mut s = self.list_start;
	let mut e = self.tagfile.len() as usize - 1;

	while s <= e {
	    let m = (s + e) / 2;
	    let (start, end) = self.line_range(m);
	    let m_item = self.item_at(start, end)?;
	    let m_id = m_item.id;

	    if id > m_id {
		s = max(end, m) + 1;
	    } else if id < m_id {
		e = start - 1;
	    } else {
		return Ok(m_item);
	    }
	}

	Err(Box::from(format!("Item {} Not Found.", id)))
    }

    fn line_range(
	&self,
	position: usize,
    ) -> (usize, usize) {
	let mut s = position;

	while s > self.list_start {
	    if self.tagfile[s] == b'@' && self.tagfile[s-1] == b'\n' {
		match self.tagfile[s+1] {
		    b'0'..=b'9' => {break;},
		    _ => (),
		}
	    }
	    s -= 1;
	}

	let mut e = s;

	while e < self.tagfile.len() {
	    if self.tagfile[e] == b'\n' {
		break;
	    }
	    e += 1;
	}

	(s, e)
    }

    fn item_at(
	&self,
	line_start: usize,
	line_end: usize,
    ) -> Result<TagItem, Box<dyn Error>> {
	let line = std::str::from_utf8(
	    &self.tagfile[line_start..line_end]
	)?;
	Ok(TagItem::try_from(line)?)
    }
}

impl TryFrom<&str> for PlainTag {
    type Error = Box<dyn Error>;

    /// Returns a PlainTag from given file path.
    ///
    /// # Errors
    ///
    /// This function will return an error if `filepath` does not
    /// already exist or when the underlying system call fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let ptag = tag::PlainTag::try_from("Tagfile");
    /// assert!(ptag.is_ok());
    /// ```

    fn try_from(filepath: &str) -> Result<PlainTag, Self::Error> {
	let tagfile = File::open(filepath)?;
	let tagfile = unsafe { Mmap::map(&tagfile)? };

	let mut list_start = 0;

	while list_start < tagfile.len() - 1 {
	    if tagfile[list_start] == b'@' {
		match tagfile[list_start + 1] {
		    b'0'..=b'9' => break,
		    _ => (),
		}
	    }

	    list_start += 1;
	}

	Ok( PlainTag {
	    tagfile,
	    list_start,
	} )
    }
}

pub struct PTagIterator<'a> {
    iter: std::str::Split<'a, char>,
}

impl <'a>Iterator for PTagIterator<'a> {
    type Item = Result<TagItem, rust_decimal::Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
	if let Some(s) = self.iter.next() {
	    if RE_TAG_ITEM.is_match(s) {
		Some(TagItem::try_from(s))
	    } else {
		self.next()
	    }
	} else {
	    None
	}
    }
}
