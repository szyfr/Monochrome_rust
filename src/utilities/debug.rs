

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{fs::File, fs::read_to_string, io::{self, Write}};
use chrono;


//= Procedures

/// Prints error string to console and to log.txt in game directory
pub fn log( input : &str ) {
	//* Print to console */
	print!("{}", input);
	let _ = io::stdout().flush();
	
	let mut logText: String;
	let now = chrono::Local::now();

	//* Attempt to load file */
	let fileResult = read_to_string("log.txt");
	if !fileResult.is_err() {
		logText = fileResult.unwrap();
	} else {
		//* Do date shit */
		logText = now.date_naive().to_string();
		logText.push('\n');
	}

	//* Save to file */
	logText.push_str(input);
	let newFile = File::create("log.txt");
	let _ = newFile.unwrap().write_all(logText.as_bytes());
}