use crate::TagItem;
use crate::TagID;

use regex::Regex;
use std::sync::LazyLock;
use std::fs::File;
use memmap2::Mmap;

static RE_TAG_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(
	r"^\s*@([0-9]+\.?[0-9]*).*$"
    ).unwrap());

pub struct PlainTag {
    tagfile: Mmap,
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
	    iter: str::from_utf8(&self.tagfile[..]).unwrap().split('\n')
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
}

impl TryFrom<&str> for PlainTag {
    type Error = Box<dyn std::error::Error>;

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

	Ok( PlainTag { tagfile } )
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
