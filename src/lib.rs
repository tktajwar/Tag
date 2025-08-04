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
	let fields = a.fields();
	assert_eq!(fields[2].concat_flags("#rust #program".to_string()),
		   Some("#hello #world #rust #program".to_string()));
	assert_eq!(fields[3].concat_flags("#rust #program".to_string()), None);
	assert_eq!(fields[2].sincat_flags("#world".to_string()),
		   Some("#hello".to_string()));
	assert_eq!(fields[2].sincat_flags("#hello #world".to_string()), None);
	assert_eq!(fields[3].sincat_flags("#world".to_string()), None);

	assert_eq!(a.flags(), Some(vec![
	    "#hello".to_string(),
	    "#world".to_string(),
	]));
	a.add_flags_to_field_no("#new #flags".to_string(), 2);
	assert_eq!(a.flags(), Some(vec![
	    "#hello".to_string(),
	    "#world".to_string(),
	    "#new".to_string(),
	    "#flags".to_string(),
	]));
    }
}
