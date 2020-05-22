use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use std::collections::VecDeque;
use std::str::Chars;

use versionator_common::BuildInfo;

mod string_value;
use string_value::string_value;

pub fn format(input: TokenStream) -> TokenStream {
	let format = parse_macro_input!(input as LitStr).value();
	let buildinfo = BuildInfo::deserialize(&std::env::var("VERSIONATOR").unwrap());

	let res = interpolate(format, &buildinfo);
	#[allow(clippy::let_and_return)]
	let output = quote!(#res);

	// println!("{}", output.to_string());
	output.into()
}

fn interpolate(format: String, buildinfo: &BuildInfo) -> String {
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
				res.push_str(&interpolate_once(n, &mut chars, buildinfo))
			}
		} else {
			res.push(c);
		}
	}
	res
}

fn interpolate_once(mut c: char, chars: &mut Chars, buildinfo: &BuildInfo) -> String {
	let mut trace = VecDeque::new();
	while c != '}' {
		c = skip_ws(c, chars).expect("Format string has an opening brace without a matching closing brace");
		if c == '?' {
			trace.push_back("?".to_string());
			c = chars
				.next()
				.expect("Format string has an opening brace without a matching closing brace");
		} else if c == '.' {
			c = chars
				.next()
				.expect("Format string has an opening brace without a matching closing brace");
			c = skip_ws(c, chars).expect("Format string has an opening brace without a matching closing brace");
			if !(c.is_alphabetic() || c == '_') {
				panic!(format!(
					"Unexpected character found while parsing identifier in format string: {:?}",
					c
				));
			}
			let mut id = String::new();
			while {
				id.push(c);
				c = chars
					.next()
					.expect("Format string has an opening brace without a matching closing brace");
				c.is_alphanumeric() || c == '_'
			} {}
			trace.push_back(id);
		} else {
			panic!(format!(
				"Unexpected character found while parsing format string: {:?}",
				c
			));
		}
	}

	string_value(buildinfo, trace)
}

fn skip_ws(mut c: char, chars: &mut Chars) -> Option<char> {
	while c.is_ascii_whitespace() {
		if let Some(n) = chars.next() {
			c = n;
		} else {
			return None;
		}
	}
	Some(c)
}
