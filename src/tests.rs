use super::*;

#[test]
fn empty_filesystem() {
	let h: HashMap<String, Tag> = HashMap::new();
	let empty = Filesystem {tags: h};

	assert_eq!(empty.tags.is_empty(), Filesystem::new().tags.is_empty());
}

#[test]
fn creating_tags() {
	let mut filesystem = Filesystem::new();
	filesystem.tags.insert("test".to_string(), Tag::new());

	let mut test = Filesystem::new();
	let _ = test.create_tag("test");

	assert_eq!(filesystem, test);
}
#[test]
fn creating_two_same_tags() {
	let mut f = Filesystem::new();
	let _ = f.create_tag("test");
	let test = f.create_tag("test");

	assert_eq!(Err(ErrorKind::AlreadyExists), test);
}
// again, misleading
#[test]
fn adding_tag_to_f() {
	let mut filesystem = Filesystem::new();
	let _ = filesystem.create_tag("test1");
	let _ = filesystem.create_tag("test2");
	let mut tag = filesystem.tags.remove("test2").unwrap();
	tag.add_file("./test");
	filesystem.tags.insert("test2".into(), tag);

	let mut test = Filesystem::new();
	let _ = test.create_tag("test1");
	let _ = test.create_tag("test2");
	test.add_tags_to_file(vec!["./test".to_string(), "test2".to_string()]);

	assert_eq!(filesystem, test);
}
#[test]
fn adding_tags_to_f() {
	let mut filesystem = Filesystem::new();
	let _ = filesystem.create_tag("test1");
	let _ = filesystem.create_tag("test2");

	let mut tag = filesystem.tags.remove("test1").unwrap();
	tag.add_file("./test");
	filesystem.tags.insert("test1".into(), tag);

	let mut tag = filesystem.tags.remove("test2").unwrap();
	tag.add_file("./test");
	filesystem.tags.insert("test2".into(), tag);


	let mut test = Filesystem::new();
	let _ = test.create_tag("test1");
	let _ = test.create_tag("test2");
	test.add_tags_to_file(vec!["./test".to_string() ,"test1".to_string(), "test2".to_string()]);

	assert_eq!(filesystem, test);
}

#[test]
fn removing_tags_from_f() {
	let mut filesystem = Filesystem::new();
	let _ = filesystem.create_tag("test1");
	let _ = filesystem.create_tag("test2");
	// add test1 tag
	filesystem.add_tags_to_file(vec!["./test".to_string(), "test1".to_string()]);

	let mut test = Filesystem::new();
	let _ = test.create_tag("test1");
	let _ = test.create_tag("test2");
	let test_vec = vec!["./test".into(), "test1".to_string(), "test2".to_string()];
	let test_vec2 = vec!["./test".into(), "test2".to_string()];
	// add test1,test2 tags
	test.add_tags_to_file(test_vec);
	// remove test2 tag
	// left with test1 tag only.
	test.remove_tags_from_file(test_vec2);

	assert_eq!(filesystem, test);
}

#[test]
fn removing_all_tags_from_f() {
	let mut filesystem = Filesystem::new();
	let _ = filesystem.create_tag("test1");
	let _ = filesystem.create_tag("test2");

	let mut test = Filesystem::new();
	let _ = test.create_tag("test1");
	let _ = test.create_tag("test2");
	test.add_tags_to_file(vec!["./test".into(), "test1".to_string(), "test2".to_string()]);
	test.untag_file(vec!["./test".into()]);

	assert_eq!(filesystem, test);
}

#[test]
fn filtering_one_tag() {
	let tag: Vec<String> = vec!["test".into(), "test2".into()];

	let mut test = Filesystem::new();
	let _ = test.create_tag("tag1");

	let _ = test.add_tags_to_file(vec!["./test".into(), "tag1".into()]); 
	let _ = test.add_tags_to_file(vec!["./test2".into(), "tag1".into()]);
	
	let test_tag = test.filter(vec!["tag1".into()]).unwrap();

	assert_eq!(tag, test_tag);
}

/// tag1 = {test, 	test2}
/// 
/// tag2 = {test, 	_}
/// 
/// tag3 = {_, 		test2}
/// 
/// files that are in both tag1 and tag2 is test.
#[test]
fn filtering_twoplus_tags() {	
	let tag: Vec<String> = vec!["test".into()];

	let mut test = Filesystem::new();
	let _ = test.create_tag("tag1");
	let _ = test.create_tag("tag2");
	let _ = test.create_tag("tag3");

	let _ = test.add_tags_to_file(vec!["./test".into(), "tag1".into(), "tag2".into()]);
	let _ = test.add_tags_to_file(vec!["./test2".into(), "tag1".into(), "tag3".into()]);

	let test = test.filter(vec!["tag1".into(), "tag2".into()]).unwrap();

	assert_eq!(tag, test);
}



#[test]
fn empty_tag() {
	let empty_tag = Tag {files: HashMap::new()};

	assert_eq!(empty_tag.files.is_empty(), Tag::new().files.is_empty());
}

#[test]
fn adding_file_to_tag() {
	let mut tag = Tag::new();
	let abs = PathBuf::from("./test").canonicalize().unwrap().display().to_string();
	tag.files.insert("test".into(), abs);

	let mut test = Tag::new();
	test.add_file("./test");

	assert_eq!(tag, test);
}

#[test]
fn adding_file_twice() {
	let mut tag = Tag::new();
	tag.add_file("./test");

	let mut test = Tag::new();
	test.add_file("./test");
	test.add_file("./test");

	assert_eq!(tag, test);
}

#[test]
fn removing_file() {
	let mut tag = Tag::new();
	tag.add_file("./test");

	let mut test = Tag::new();
	test.add_file("./test");
	test.add_file("./test2");
	test.remove_file("./test2");

	assert_eq!(tag, test);
}
#[test]
fn removing_file_twice() {
	let mut tag = Tag::new();
	tag.add_file("./test");

	let mut test = Tag::new();
	test.add_file("./test");
	test.add_file("./test2");

	test.remove_file("./test2");
	test.remove_file("./test2");

	assert_eq!(tag, test);
}
