mod tag;

pub use crate::tag::TagID;
pub use crate::tag::TagField;
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

    #[test]
    fn field_types() {
	let a = TagItem::from("@1.0 | My Title | #hello #world | :src: code | :invalid | #valid-flag | #inva!!lid");
	let fields = a.fields();
	assert_eq!(fields[0], TagField::ID("@1.0"));
	assert_eq!(fields[1], TagField::Title("My Title"));
	assert_eq!(fields[2], TagField::Flags("#hello #world"));
	assert_eq!(fields[3], TagField::Attribute(":src: code"));
	assert_eq!(fields[4], TagField::Invalid);
	assert_eq!(fields[5], TagField::Flags("#valid-flag"));
	assert_eq!(fields[6], TagField::Invalid);
    }
}
