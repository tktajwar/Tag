use std::fs::File;
use memmap2::Mmap;

pub struct PlainTag {
    tagfile: Mmap,
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
