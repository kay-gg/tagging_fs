mod tests;

use std::{collections::HashMap, env, io::ErrorKind, path::PathBuf, ptr::NonNull};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Filesystem {
	// <Tag name, Tag>
	tags: HashMap<String, Tag>,
}
impl Filesystem {
	/// Creates a ```Filesystem``` with no ```Tag```s
	pub fn new() -> Filesystem {
		let h: HashMap<String, Tag> = HashMap::new();
		let f: Filesystem = Filesystem {tags: h};

		return f;
	}

	/// Adds a ```Tag``` with ```name``` to the ```Filesystem```
	pub fn create_tag(&mut self, name: &str) -> Result<(), ErrorKind> {
		match self.tags.contains_key(name) {
			false => {
				let t: Tag = Tag::new();
				self.tags.insert(name.to_owned(), t);
				return Ok(());
			}
			true => Err(ErrorKind::AlreadyExists)
		}
	}
	/// Adds file to each tag in ```tags```
	/// 
	/// path is 0, for example
	/// {path} {tag1} ... {tagn}
	pub fn add_tags_to_file(&mut self, mut tags_vec: Vec<String>) {
	// name is misleading because we arent actually adding tags to the files itself,
	// but it sounds nicer this way. on the fence about changing
		let path = tags_vec.remove(0);

		for t in tags_vec {
			if let Some(tag) = self.tags.get_mut(&t) {
				tag.add_file(&path);
			} else {
				println!("Tag: {} not found.", t);
			}
		}
	}
	/// Removes file to each tag in ```tags```
	/// 
	/// path is 0, for example
	/// {path} {tag1} ... {tagn}
	pub fn remove_tags_from_file(&mut self, mut tags_vec: Vec<String>) {
		let path = tags_vec.remove(0);

		for t in tags_vec {
			if let Some(tag) = self.tags.get_mut(&t) {
				tag.remove_file(&path);
			} else {
				println!("Tag: {} not found.", t);
			}
		}
	}
	/// Removes all tags from a file
	pub fn untag_file(&mut self, tags_vec: Vec<String>) {
		// this is ugly

		// for each tag
		for t in self.tags.iter_mut() {
			// if tag has key
			if t.1.files.contains_key(&Tag::get_filename(&tags_vec[0])) {
				t.1.remove_file(&tags_vec[0]);
			}
		}
	}
	/// Takes tags to filter by, ```tags_vec```,
	/// 
	/// Removes files from each tag that are not in all tags
	/// 
	/// Returns a ```Vec<String>``` of the filenames that have all the tags
	pub fn filter(&self, tags_vec: Vec<String>) -> Result<Vec<String>, ErrorKind> {
		let mut intersection = Vec::new();
		let mut hashmap: HashMap<&String, i32> = HashMap::new();

		// confirm tags exist in Filesystem
		for tags in tags_vec.iter() {
			if let None = self.tags.get(tags) {
				return Err(ErrorKind::NotFound);
			}
		}
		// add to hashmap
		// hashmap contains <files, numbers of times files added>
		for tags in tags_vec.iter() {
			let tag_in_fs = self.tags.get(tags).unwrap();
			for files in tag_in_fs.files.iter() {
				if hashmap.contains_key(files.0) {
					let key = hashmap.get_mut(files.0).unwrap();
					*key += 1;
				} else {
					hashmap.insert(files.0, 1);
				}
			}
		}
		// if number of times = number of tags, they intersect on all tags
		for files in hashmap {
			if files.1 as usize == tags_vec.len() {
				intersection.push(files.0.into());
			}
		}

		return Ok(intersection);
	}

	/// Returns files in filesystem.
	/// 
	/// If none, there are no files in the filesystem.
	/// 
	/// As this is used in frontend only, it returns file names only, not paths.
	pub fn return_files(&self) -> Option<Vec<String>> {
		// could be changed to a hashset
		let mut hash: HashMap<String, i8> = HashMap::new();
		// for each file in tag, insert into hashmap, this removes duplicate names.
		for tags in self.tags.values() {
			for files in tags.files.iter() {
				hash.insert(files.0.clone(), 0);
			}
		}
		// push filenames into a vec, now that there are no repeats
		let mut v: Vec<String> = Vec::new();
		for files in hash.keys() {
			v.push(files.into());
		}
		
		if v.len() > 0 {
			return Some(v);
		} else {
			return None;
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tag {
	// <filename, absolute path>
	files: HashMap<String, String>,
}
impl Tag {
	/// Returns an empty Tag
	fn new() -> Tag {
		let t: Tag = Tag {files: HashMap::new()};
		return t;
	}
	
	fn add_file(&mut self, path: &str) {
		let absolute_path = Tag::get_abs_path(&path);
		let filename = Tag::get_filename(&path);

		self.files.insert(filename, absolute_path);
	}
	
	fn remove_file(&mut self, path: &str) {
		let filename = Tag::get_filename(&path);

		let _ = self.files.remove(&filename);
	}

	// paths only work with / seperators.... might be a problem but is working for rn
	// might want to find a way to make this not use a PathBuf.display() bc it doesnt look good in tags file
	fn get_abs_path(path: &str) -> String {
		return PathBuf::from(path).canonicalize().unwrap().display().to_string();
	}
	pub fn get_filename(path: &str) -> String {
		let mut x: Vec<&str> = Vec::new();
		if env::consts::OS == "windows" {
			x = path.split("\\").collect();
		} else {
			x = path.split("/").collect();
		}
		let x = x.last().unwrap().to_owned();

		return String::from(x);
	}
}

