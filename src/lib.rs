mod tag;

pub use crate::tag::TagID;
pub use crate::tag::TagField;
pub use crate::tag::TagItem;
pub use crate::tag::TagMap;

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
	assert_eq!(fields[4], TagField::Invalid(":invalid"));
	assert_eq!(fields[5], TagField::Flags("#valid-flag"));
	assert_eq!(fields[6], TagField::Invalid("#inva!!lid"));
    }

    #[test]
    fn tag_flags() {
	let a = TagItem::from("@1.0 | My Title | #hello #world | :src: code | :invalid | #valid-flag | #inva!!lid");
	let fields = a.fields();
	assert_eq!(fields[2].flags(), Some(vec!["#hello".to_string(),
						"#world".to_string(),
	]));
	assert_eq!(fields[3].flags(), None);
	assert_eq!(a.flags(), Some(vec!["#hello".to_string(),
					"#world".to_string(),
					"#valid-flag".to_string(),
	]));
	assert!(a.has_flag("#hello".to_string()));
	assert!(a.has_flag("#valid-flag".to_string()));
	assert!(!(a.has_flag("#test".to_string())));
	assert!(a.has_flags(vec![
	    "#hello".to_string(),
	    "#world".to_string(),
	    "#valid-flag".to_string(),
	]));

	let b = TagItem::from("@1.0 | Item with no flags");
	assert_eq!(b.flags(), None);
	assert!(!(b.has_flag("#hello".to_string())));
	assert!(!(b.has_flags(vec![
	    "#hello".to_string(),
	    "#world".to_string(),
	    "#valid-flag".to_string(),
	])));
    }

    #[test]
    fn tag_attributes() {
	let a = TagItem::from("@1.0 | My Title | #hello #world | :src: code | :null:");
	let fields = a.fields();
	assert_eq!(fields[1].attribute_key(), None);
	assert_eq!(fields[1].attribute_value(), None);
	assert_eq!(fields[3].attribute_key(), Some(":src:".to_string()));
	assert_eq!(fields[3].attribute_value(), Some("code".to_string()));
	assert_eq!(fields[4].attribute_key(), Some(":null:".to_string()));
	assert_eq!(fields[4].attribute_value(), None);
	assert_eq!(fields[1].attribute_key_value(), None);
	assert_eq!(fields[3].attribute_key_value(), Some((Some(":src:".to_string()),
							  Some("code".to_string()))));
	assert_eq!(fields[4].attribute_key_value(), Some((Some(":null:".to_string()),
							  None)));
	assert_eq!(a.attributes(), Some(vec![
	    (Some(":src:".to_string()), Some("code".to_string())),
	    (Some(":null:".to_string()), None),
	]));
	assert_eq!(a.fetch_attribute(":src:".to_string()), Some((Some(":src:".to_string()),
								 Some("code".to_string()))));
	assert!(a.has_attribute(":null:".to_string()));
	assert!(a.match_attribute((Some(":src:".to_string()),
				   Some("code".to_string()))));
	assert!(!(a.match_attribute((Some(":src:".to_string()),
				   Some("unkown".to_string())))));
	assert!(a.match_attribute((Some(":null:".to_string()),
				   None)));
	let b = TagItem::from("@1.0 | Item with no attributes");
	assert_eq!(b.attributes(), None);
	assert_eq!(b.fetch_attribute(":src:".to_string()), None);
	assert!(!(b.has_attribute(":null:".to_string())));
	assert!(!(b.match_attribute((Some(":src:".to_string()),
				   Some("code".to_string())))));
    }

    #[test]
    fn tag_flags_modification() {
	let mut a = TagItem::from("@1.0 | My Title | #hello #world | :src: code | :null:");
	let mut b = TagItem::from("@2.0 | Multiple flag fields | #A #B | #B #C");
	let fields = a.fields();
	assert_eq!(fields[2].concat_flags("#rust #program"),
		   Some("#hello #world #rust #program".to_string()));
	assert_eq!(fields[3].concat_flags("#rust #program"), None);
	assert_eq!(fields[2].sincat_flags("#world"),
		   Some("#hello".to_string()));
	assert_eq!(fields[2].sincat_flags("#hello #world"), None);
	assert_eq!(fields[3].sincat_flags("#world"), None);

	assert_eq!(a.flags(), Some(vec![
	    "#hello".to_string(),
	    "#world".to_string(),
	]));
	a.add_flags_to_field_no("#new #flags", 2);
	assert_eq!(a.flags(), Some(vec![
	    "#hello".to_string(),
	    "#world".to_string(),
	    "#new".to_string(),
	    "#flags".to_string(),
	]));
	a.remove_flags_from_field_no("#hello #new", 2);
	assert_eq!(a.flags(), Some(vec![
	    "#world".to_string(),
	    "#flags".to_string(),
	]));
	a.add_flags("#hola #renew");
	assert_eq!(a.flags(), Some(vec![
	    "#world".to_string(),
	    "#flags".to_string(),
	    "#hola".to_string(),
	    "#renew".to_string(),
	]));
	a.remove_flags("#flags #renew");
	assert_eq!(a.flags(), Some(vec![
	    "#world".to_string(),
	    "#hola".to_string(),
	]));
	b.add_flags("#D");
	assert_eq!(b.flags(), Some(vec![
	    "#A".to_string(),
	    "#B".to_string(),
	    "#B".to_string(),
	    "#C".to_string(),
	    "#D".to_string(),
	]));
	b.remove_flags("#B");
	assert_eq!(b.flags(), Some(vec![
	    "#A".to_string(),
	    "#C".to_string(),
	    "#D".to_string(),
	]));
	assert_eq!(b.fields()[2].flags(), Some(vec!["#A".to_string(),]));
    }

    #[test]
    fn tag_attributes_modification() {
	let mut a = TagItem::from("@1.0 | My Title | #hello #world | :src: code | :null:");
	a.set_attribute_at_field_no((Some(":src:"), Some("lib.rs")), 3);
	assert!(a.match_attribute((Some(":src:".to_string()),
				   Some("lib.rs".to_string()))));
	a.set_attribute((Some(":src:"), Some("code")));
	assert!(a.match_attribute((Some(":src:".to_string()),
				   Some("code".to_string()))));
	a.set_attribute((Some(":attr:"), Some("val")));
	assert!(a.match_attribute((Some(":attr:".to_string()),
				   Some("val".to_string()))));
	a.remove_attribute(":attr:");
	assert!(!(a.has_attribute(":attr:".to_string())));
	a.remove_attribute(":null:");
	assert!(!(a.has_attribute(":null:".to_string())));
    }

    #[test]
    fn tag_map() {
	let tag_file = "\
	@1.0  | First Item    | :file: introduction.txt\n\
	@1.1  | Sub-item?     | :file: special.txt | #special\n\
	@1.11 | Sub-sub-item? | :file: extra.txt   | #special\n\
	@2.0  | Second Item   | :file: body.txt\n\
	@3.0  | The End       | :file: credits.txt\
	";
	let tag_map = TagMap::try_from(tag_file);
	assert!(tag_map.is_ok());

	let tag_file_unsorted = "\
	@2.0  | Second Item\n\
	@1.0  | First Item\n\
	";
	let tag_map_unsorted = TagMap::try_from(tag_file_unsorted);
	assert!(tag_map_unsorted.is_err());

	let tag_file_with_duplicate = "\
	@1.0  | First Item\n\
	@1.0  | First Item\n\
	";
	let tag_map_with_duplicate = TagMap::try_from(tag_file_with_duplicate);
	assert!(tag_map_with_duplicate.is_err());
    }
}
