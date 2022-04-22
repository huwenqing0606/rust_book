// sample code on common collections

fn main() {
	println!("--- test vector ---");
	vector();
	println!("--- test string ---");
	string();
	println!("--- test hash map ---");
	hash_map();
}


fn vector() {
	let mut v: Vec<i32> = Vec::new();
	v.push(5);
	let zero: &i32 = &v[0];
	println!("vector = {}", v[0]);
	println!("vector's zero-th element is = {}", zero);
	
	match v.get(2) {
		Some(element) => {println!("the {}-th element is {}", 3, element)},
		None => {println!("the {}-th element does not exist!", 3)},
	};
	
	let v = vec![100, 32, 57];
	for i in &v {
		println!("the element is {}", i);
	}
	
	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}
	for i in &v {
		println!("the element is {}", i);
	}
	
	enum SpreadSheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}
	
	let row = vec![
		SpreadSheetCell::Int(3),
		SpreadSheetCell::Float(0.1),
		SpreadSheetCell::Text(String::from("hello"))];
	
	match &row[0] {
		SpreadSheetCell::Int(value) => {println!("int is {}", value);},
		SpreadSheetCell::Float(value) => {println!("float is {}", value);},
		SpreadSheetCell::Text(value) => {println!("text is {}", value);},
	}
}

fn string() {
	let mut s = String::new();
	println!("string is {}", s);
	s = "initial contents".to_string();
	s.push_str(" bar");
	println!("string is '{}'", s);
	
	let s2 = "bar";
	s.push_str(s2);
	println!("string is '{}'", s);
	
	let s3 = s + &s2;
	println!("string is '{}'", s3);
	
	s = format!("{}-{}", s2, s3);
	println!("string is '{}'", s);
	
	let slice = &s[0..4];
	println!("string slice is '{}'", slice);
	
	for c in s2.chars() {
		println!("{}", c);
	}
	
	for c in s2.bytes() {
		println!("{}", c);
	}
}

fn hash_map() {
	use std::collections::HashMap;
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	
	println!("scores = {:?}", scores);
	
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	
	let mut scores: HashMap <_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
	
	println!("scores = {:?}", scores);
	
	let field_name = String::from("faorite color");
	let field_value = String::from("Blue");
	
	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	println!("map = {:?}", map);
	// println!("field_name = {}, field_value = {}", field_name, field_value);
		// field_name and field_value are out of their lifetimes once being borrowed into HashMap
	
	let team_name = String::from("Purple");
	let score = scores.get(&team_name);
	match score {
		Some(value) => {println!("team {} has score {:?}", team_name, value);},
		None => {println!("team {} do not have score", team_name);},
	}
	
	for (key, value) in &scores {
		println!("key = {}, value = {}", key, value);
	}
	
	println!("scores = {:?}", scores);
	scores.insert(String::from("Blue"), 15);
	println!("scores = {:?}", scores);
	
	scores.entry(String::from("Blue")).or_insert(20);
	println!("scores = {:?}", scores);
	
	let text = "Hello world wonderful world";
	let mut map = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("text '{}' word count = {:?}", text, map);
}