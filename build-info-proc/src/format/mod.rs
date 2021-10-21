use anyhow::Result;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, emit_error};
use quote::quote;
use syn::parse_macro_input;

use std::str::Chars;

use build_info_common::BuildInfo;

mod eval;
use eval::Eval;

mod syntax;

mod types;
use types::Type;

mod value;
use value::{FormatSpecifier, Value, OP_ARRAY_INDEX, OP_FIELD_ACCESS, OP_TUPLE_INDEX};

pub fn format(input: TokenStream, _build_info: BuildInfo) -> TokenStream {
	let syntax = parse_macro_input!(input as syntax::Syntax);
	let values: Result<Vec<Box<dyn Value>>> = syntax.args.iter().map(|v| v.eval()).collect();
	let values = values.unwrap_or_else(|err| abort_call_site!(err.to_string()));

	let str = if values.is_empty() {
		super::deserialize_build_info().to_string()
	} else {
		let format = values[0].as_any().downcast_ref::<String>().unwrap_or_else(
			|| abort_call_site!("Could not interpret first argument as a string"; note = "It is {:#?}", &*values[0];),
		);
		interpolate(format, &values[1..], Span::call_site())
	};
	let output = quote!(#str);

	// println!("{}", output.to_string());
	output.into()
}

const CLOSING_BRACE_EXPECTED: &str = "Invalid format string: unmatched `{` found";
const CLOSING_BRACE_NOTE: &str = "If you intended to print `{`, you can escape it using `{{`.";

fn interpolate(format: &str, args: &[Box<dyn Value>], span: Span) -> String {
	let mut res = String::with_capacity(format.len());
	let mut implicit_position = 0usize;
	let mut argument_used = Vec::new();
	argument_used.resize(args.len(), false);

	let mut chars = format.chars();
	while let Some(c) = chars.next() {
		if c == '{' {
			let n = chars
				.next()
				.unwrap_or_else(|| abort!(span, CLOSING_BRACE_EXPECTED; note = CLOSING_BRACE_NOTE;));
			if n == '{' {
				res.push(c);
			} else {
				interpolate_once(
					&mut res,
					n,
					&mut chars,
					args,
					&mut argument_used,
					&mut implicit_position,
					span,
				);
			}
		} else if c == '}' {
			let n = chars.next();
			if n == Some('}') {
				res.push(c);
			} else {
				abort!(
					span, "Invalid format string: unmatched `}` found";
					note = "If you intended to print `}`, you can escape it using `}}`.";
				)
			}
		} else {
			res.push(c);
		}
	}

	for (i, used) in argument_used.iter().enumerate() {
		if !used {
			emit_error!(span,
				"Parameter {} is not used in format string.", i;
				note = "Positional arguments are zero-based";
			);
		}
	}

	res
}

fn interpolate_once(
	buffer: &mut String,
	mut c: char,
	chars: &mut Chars,
	args: &[Box<dyn Value>],
	argument_used: &mut [bool],
	implicit_position: &mut usize,
	span: Span,
) {
	let mut explicit_position = None;
	if c.is_ascii_digit() {
		let mut acc = 0;
		#[allow(clippy::blocks_in_if_conditions)]
		while {
			acc = acc * 10 + c.to_digit(10).unwrap() as usize;
			c = chars
				.next()
				.unwrap_or_else(|| abort!(span, CLOSING_BRACE_EXPECTED; note = CLOSING_BRACE_NOTE;));
			c.is_ascii_digit()
		} {}
		explicit_position = Some(acc);
	} else if c.is_alphabetic() {
		abort!(
			span,
			"Keyword arguments are currently not supported in build_info::format"
		);
	}

	let arg = if let Some(pos) = explicit_position {
		let arg = args.get(pos).unwrap_or_else(|| {
			abort!(span,
				"Invalid reference to positional argument {} ({} arguments were given)", pos, args.len();
				note = "Positional arguments are zero-based";
			)
		});
		argument_used[pos] = true;
		arg
	} else {
		let arg = args.get(*implicit_position).unwrap_or_else(|| {
			abort!(span,
				"Invalid implicit reference to positional argument {} ({} arguments were given)",
				*implicit_position,
				args.len();
				note = "Positional arguments are zero-based";
			)
		});
		argument_used[*implicit_position] = true;
		*implicit_position += 1;
		arg
	};

	let mut debug = false;
	let mut alternate = false;
	if c == ':' {
		c = chars
			.next()
			.unwrap_or_else(|| abort!(span, CLOSING_BRACE_EXPECTED; note = CLOSING_BRACE_NOTE;));
		if c == '#' {
			alternate = true;
			c = chars
				.next()
				.unwrap_or_else(|| abort!(span, CLOSING_BRACE_EXPECTED; note = CLOSING_BRACE_NOTE;));
		}
		if c == '?' {
			debug = true;
			c = chars
				.next()
				.unwrap_or_else(|| abort!(span, CLOSING_BRACE_EXPECTED; note = CLOSING_BRACE_NOTE;));
		}
	}

	if c == '}' {
		if debug {
			if alternate {
				arg.format(buffer, FormatSpecifier::DebugAlt);
			} else {
				arg.format(buffer, FormatSpecifier::Debug);
			}
		} else {
			debug_assert!(!alternate);
			arg.format(buffer, FormatSpecifier::Default);
		}
	} else {
		abort!(span,
			"Unexpected character {:?} in format specifier.", c;
			note = CLOSING_BRACE_NOTE;
		);
	}
}
