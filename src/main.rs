extern crate rust_parse_describe;

use rust_parse_describe::*;

use std::io::{self, Read};

fn main() {
		
	let mut buffer = String::new();
	let result = io::stdin().read_to_string(&mut buffer);
	
	match result {
		Err(err) => println!("Error: {}.", err),
		Ok(_) => parse_describe::parse_analysis_forStdout(&buffer),
	}
	
//	parse_describe::parse_analysis("fn foo(");
}