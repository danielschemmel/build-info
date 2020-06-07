use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use std::str::Chars;

use build_info_common::BuildInfo;

mod indexed_string_value;
use indexed_string_value::indexed_string_value;

pub fn format(input: TokenStream, build_info: BuildInfo) -> TokenStream {
	let format = parse_macro_input!(input as LitStr).value();

	let res = interpolate(format, &build_info);
	#[allow(clippy::let_and_return)]
	let output = quote!(#res);

	// println!("{}", output.to_string());
	output.into()
}

fn interpolate(format: String, build_info: &BuildInfo) -> String {
	let mut chars = format.chars();
	let mut res = String::new();
	while let Some(c) = chars.next() {
		if c == '{' {
			let n = chars
				.next()
				.expect("Format string has an opening brace without a matching closing brace");
			if n == '{' {
				res.push(c);
			} else {
				res.push_str(&interpolate_once(n, &mut chars, build_info))
			}
		} else if c == '}' {
			let n = chars
				.next()
				.expect("Format string has an closing brace without a matching opening brace");
			if n == '}' {
				res.push(c);
			} else {
				panic!("Format string has an closing brace without a matching opening brace")
			}
		} else {
			res.push(c);
		}
	}
	res
}

#[derive(Debug)]
pub(crate) enum Index {
	Unwrap,
	Field(String),
	Function(String, Vec<String>),
}

const CLOSING_BRACE_EXPECTED: &str = "Format string has an opening brace without a matching closing brace";

fn interpolate_once(mut c: char, chars: &mut Chars, build_info: &BuildInfo) -> String {
	let mut trace = Vec::new();
	while c != '}' {
		c = skip_ws(c, chars);
		if c == '?' {
			trace.push(Index::Unwrap);
			c = chars.next().expect(CLOSING_BRACE_EXPECTED);
		} else if c == '.' {
			c = chars.next().expect(CLOSING_BRACE_EXPECTED);
			c = skip_ws(c, chars);

			let (n, id) = parse_id(c, chars);
			c = n;

			c = skip_ws(c, chars);

			if c == '(' {
				c = chars.next().expect(CLOSING_BRACE_EXPECTED);
				let args = Vec::new();
				loop {
					c = skip_ws(c, chars);
					if c == ')' {
						c = chars.next().expect(CLOSING_BRACE_EXPECTED);
						break;
					} else {
						panic!(format!(
							"Unexpected character found inside function call arguments while parsing format string: {:?}",
							c
						));
					}
				}
				trace.push(Index::Function(id, args));
			} else {
				trace.push(Index::Field(id));
			}
		} else {
			panic!(format!(
				"Unexpected character found while parsing format string: {:?}",
				c
			));
		}
	}

	indexed_string_value(build_info, &trace)
}

fn parse_id(mut c: char, chars: &mut Chars) -> (char, String) {
	if !(c.is_alphabetic() || c == '_') {
		panic!(format!(
			"Unexpected character found while parsing identifier in format string: {:?}",
			c
		));
	}

	let mut id = String::new();
	while {
		id.push(c);
		c = chars.next().expect(CLOSING_BRACE_EXPECTED);
		c.is_alphanumeric() || c == '_'
	} {}

	(c, id)
}

fn skip_ws(mut c: char, chars: &mut Chars) -> char {
	while c.is_ascii_whitespace() {
		c = chars.next().expect(CLOSING_BRACE_EXPECTED);
	}
	c
}
