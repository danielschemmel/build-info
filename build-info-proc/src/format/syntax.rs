use num_bigint::BigInt;
use proc_macro2::Span;
use syn::{bracketed, parenthesized, parse, Ident, LitInt, LitStr, Token};

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub(crate) struct Meta {
	pub(crate) span: Span,
}

impl PartialOrd for Meta {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Meta {
	fn cmp(&self, _other: &Self) -> Ordering {
		Ordering::Equal
	}
}

impl PartialEq for Meta {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl Eq for Meta {}

impl Hash for Meta {
	fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl Default for Meta {
	fn default() -> Self {
		Self {
			span: Span::call_site(),
		}
	}
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Syntax {
	pub(crate) args: Vec<Expr>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Expr {
	pub(crate) atom: AtomicExpr,
	pub(crate) suffixes: Vec<Suffix>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum AtomicExpr {
	BuildInfo(Meta),
	LitInt(BigInt, Meta),
	LitStr(String, Meta),
	Parenthesized(Box<Expr>, Meta),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum Suffix {
	Unwrap,
	Field(String),
	TupleIndex(u32),
	ArrayIndex(Box<Expr>),
	FunctionCall(String, Vec<Expr>),
}

impl parse::Parse for Syntax {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let args = parse_arguments(&input)?;

		Ok(Self { args })
	}
}

fn parse_arguments(input: parse::ParseStream) -> parse::Result<Vec<Expr>> {
	let mut args = Vec::new();
	if !input.is_empty() {
		args.push(input.parse::<Expr>()?);
		parse_trailing_arguments_impl(&mut args, input)?;
	}
	Ok(args)
}

/*
fn parse_trailing_arguments(input: parse::ParseStream) -> parse::Result<Vec<Expr>> {
	let mut args = Vec::new();
	parse_trailing_arguments_impl(&mut args, input)?;
	Ok(args)
}
*/

fn parse_trailing_arguments_impl(args: &mut Vec<Expr>, input: parse::ParseStream) -> parse::Result<()> {
	while !input.is_empty() {
		input.parse::<Token![,]>()?;
		args.push(input.parse::<Expr>()?);
	}
	Ok(())
}

impl parse::Parse for AtomicExpr {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let lookahead = input.lookahead1();
		if lookahead.peek(Token![$]) {
			let token = input.parse::<Token![$]>()?;
			Ok(AtomicExpr::BuildInfo(Meta { span: token.spans[0] }))
		} else if lookahead.peek(syn::token::Paren) {
			let expr;
			parenthesized!(expr in input);
			Ok(AtomicExpr::Parenthesized(
				Box::new(expr.parse::<Expr>()?),
				Meta { span: expr.span() },
			))
		} else if lookahead.peek(LitInt) {
			let lit_int = input.parse::<LitInt>()?;
			if lit_int.suffix() != "" {
				return Err(syn::Error::new(
					lit_int.span(),
					"Integer suffix is not supported in [build-info] yet",
				));
			}
			Ok(AtomicExpr::LitInt(
				lit_int.base10_parse::<BigInt>()?,
				Meta { span: lit_int.span() },
			))
		} else if lookahead.peek(LitStr) {
			let lit_str = input.parse::<LitStr>()?;
			Ok(AtomicExpr::LitStr(lit_str.value(), Meta { span: lit_str.span() }))
		} else {
			println!("{}", input);
			Err(lookahead.error())
		}
	}
}

impl parse::Parse for Expr {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let atom = input.parse::<AtomicExpr>()?;

		let mut suffixes = Vec::new();
		while !input.is_empty() {
			let lookahead = input.lookahead1();
			if lookahead.peek(Token![,]) {
				break;
			} else if lookahead.peek(Token![?]) {
				input.parse::<Token![?]>()?;
				suffixes.push(Suffix::Unwrap);
			} else if lookahead.peek(Token![.]) {
				input.parse::<Token![.]>()?;
				let lookahead = input.lookahead1();
				if lookahead.peek(Ident) {
					let id = input.parse::<Ident>()?;

					let lookahead = input.lookahead1();
					if lookahead.peek(syn::token::Paren) {
						let arguments;
						parenthesized!(arguments in input);
						let arguments = parse_arguments(&arguments)?;
						suffixes.push(Suffix::FunctionCall(id.to_string(), arguments));
					} else {
						suffixes.push(Suffix::Field(id.to_string()));
					}
				} else if lookahead.peek(LitInt) {
					let tuple_index = input.parse::<LitInt>()?;
					suffixes.push(Suffix::TupleIndex(tuple_index.base10_parse()?));
				} else {
					return Err(lookahead.error());
				}
			} else if lookahead.peek(syn::token::Bracket) {
				let expr;
				bracketed!(expr in input);
				let expr = expr.parse::<Expr>()?;
				suffixes.push(Suffix::ArrayIndex(Box::new(expr)));
			} else {
				return Err(lookahead.error());
			}
		}

		Ok(Self { atom, suffixes })
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use quote::quote;

	#[test]
	fn no_format() {
		let format = "This is a $test".to_string();
		let ast = quote! {#format};
		let result = syn::parse2::<Syntax>(ast).unwrap();
		assert_eq!(
			result,
			Syntax {
				args: vec![Expr {
					atom: AtomicExpr::LitStr(format, Meta::default()),
					suffixes: vec![],
				}],
			}
		);
	}

	#[test]
	fn format_self() {
		let format = "{}".to_string();
		let ast = quote! {#format, $};
		let result = syn::parse2::<Syntax>(ast).unwrap();
		assert_eq!(
			result,
			Syntax {
				args: vec![
					Expr {
						atom: AtomicExpr::LitStr(format, Meta::default()),
						suffixes: vec![],
					},
					Expr {
						atom: AtomicExpr::BuildInfo(Meta::default()),
						suffixes: vec![]
					}
				]
			}
		);
	}

	#[test]
	fn format_suffixes() {
		let format = "{}".to_string();
		let ast = quote! {#format, $.foo().7[0x0_C].foo};
		let result = syn::parse2::<Syntax>(ast).unwrap();
		assert_eq!(
			result,
			Syntax {
				args: vec![
					Expr {
						atom: AtomicExpr::LitStr(format, Meta::default()),
						suffixes: vec![],
					},
					Expr {
						atom: AtomicExpr::BuildInfo(Meta::default()),
						suffixes: vec![
							Suffix::FunctionCall("foo".to_string(), vec![]),
							Suffix::TupleIndex(7),
							Suffix::ArrayIndex(Box::new(Expr {
								atom: AtomicExpr::LitInt(12.into(), Meta::default()),
								suffixes: vec![],
							})),
							Suffix::Field("foo".to_string())
						]
					}
				],
			}
		);
	}

	#[test]
	fn format_trailing_comma() {
		let format = "{}".to_string();
		let ast = quote! {#format,};
		let result = syn::parse2::<Syntax>(ast);
		assert!(result.is_err());
	}
}
