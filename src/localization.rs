

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::collections::HashMap;
use std::fs::read_to_string;
use crate::settings;
use crate::utilities::debug;


//= Procedures

/// Loads all language strings into Hashmap using input language
pub fn load( language : &settings::Language ) -> HashMap<String, String> {
	let mut output : HashMap<String, String> = HashMap::new();

	//* Attempt to open file */
	let fileResult = read_to_string(format!("data/localization/{}.json", language));
	if fileResult.is_err() {
		debug::log("[ERROR] - Failed to load localization file.");
		return output;
	}

	//* Convert to Json */
	let str = fileResult.unwrap();
	let jsonFile: serde_json::Value = serde_json::from_str(&str).unwrap();

	//* Parse */
	for val in jsonFile.as_object().unwrap() {
		output.insert(val.0.to_string(), val.1.as_str().unwrap().to_string());
	}

	return output;
}