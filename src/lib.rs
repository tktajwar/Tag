mod tag;

pub use crate::tag::TagID;
pub use crate::tag::TagItem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
	let a = TagID::from("@1.0");
	let b = TagID::generate_next(&a);
	let a1 = TagID::generate_between(&a, &b);
	let a2 = TagID::generate_between(&a1, &b);

	assert!(a < a1 && a1 < a2 && a2 < b);
    }
}
