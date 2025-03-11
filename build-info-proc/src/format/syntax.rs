use std::{
	cmp::Ordering,
	hash::{Hash, Hasher},
};

use num_bigint::BigInt;
use proc_macro2::Span;
use syn::{Ident, LitBool, LitChar, LitInt, LitStr, Token, braced, bracketed, parenthesized, parse};

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
	pub(crate) args: Vec<(Option<Ident>, Expr)>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Expr {
	pub(crate) atom: AtomicExpr,
	pub(crate) suffixes: Vec<Suffix>,
}

impl Expr {
	pub fn meta(&self) -> &Meta {
		self.atom.meta()
	}
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum AtomicExpr {
	BuildInfo(Meta),
	LitBool(bool, Meta),
	LitInt(BigInt, Meta),
	LitChar(char, Meta),
	LitStr(String, Meta),
	Parenthesized(Box<Expr>, Meta),
	FunctionCall(String, Vec<Expr>, Meta),
	MacroCall(String, Vec<(Option<Ident>, Expr)>, Meta),
}

impl AtomicExpr {
	pub fn meta(&self) -> &Meta {
		match self {
			AtomicExpr::BuildInfo(meta) => meta,
			AtomicExpr::LitBool(_, meta) => meta,
			AtomicExpr::LitInt(_, meta) => meta,
			AtomicExpr::LitChar(_, meta) => meta,
			AtomicExpr::LitStr(_, meta) => meta,
			AtomicExpr::Parenthesized(_, meta) => meta,
			AtomicExpr::FunctionCall(.., meta) => meta,
			AtomicExpr::MacroCall(.., meta) => meta,
		}
	}
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
		let args = parse_named_arguments(input)?;

		Ok(Self { args })
	}
}

fn parse_simple_arguments(input: parse::ParseStream) -> parse::Result<Vec<Expr>> {
	let result = input.parse_terminated(parse::Parse::parse, Token![,])?;
	Ok(result.into_pairs().map(|pair| pair.into_tuple().0).collect())
}

fn parse_named_arguments(input: parse::ParseStream) -> parse::Result<Vec<(Option<Ident>, Expr)>> {
	let result = input.parse_terminated(parse_named_argument, Token![,])?;

	let mut named = Vec::new();
	for (name, expr) in result.iter() {
		if name.is_none() {
			if !named.is_empty() {
				// TODO: use the spans in `named` to also point to the previous named args
				return Err(syn::Error::new(
					expr.meta().span,
					"Positional arguments must be before named arguments",
				));
			}
		} else {
			named.push(expr.meta().span);
		}
	}

	Ok(result.into_pairs().map(|pair| pair.into_tuple().0).collect())
}

fn parse_named_argument(input: parse::ParseStream) -> parse::Result<(Option<Ident>, Expr)> {
	if input.peek(Ident) && input.peek2(Token![=]) {
		let id = input.parse::<Ident>()?;
		input.parse::<Token![=]>()?;
		Ok((Some(id), input.parse()?))
	} else {
		Ok((None, input.parse()?))
	}
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
		} else if lookahead.peek(LitBool) {
			let lit_bool = input.parse::<LitBool>()?;
			Ok(AtomicExpr::LitBool(lit_bool.value, Meta { span: lit_bool.span }))
		} else if lookahead.peek(LitChar) {
			let lit_char = input.parse::<LitChar>()?;
			Ok(AtomicExpr::LitChar(lit_char.value(), Meta { span: lit_char.span() }))
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
		} else if lookahead.peek(Ident) {
			let id = input.parse::<Ident>()?;

			let lookahead = input.lookahead1();
			if lookahead.peek(syn::token::Paren) {
				let arguments;
				parenthesized!(arguments in input);
				let (arguments, span) = (parse_simple_arguments(&arguments)?, arguments.span());
				Ok(AtomicExpr::FunctionCall(id.to_string(), arguments, Meta { span }))
			} else if lookahead.peek(Token![!]) {
				input.parse::<Token![!]>()?;
				let lookahead = input.lookahead1();
				let (arguments, span) = if lookahead.peek(syn::token::Paren) {
					let arguments;
					parenthesized!(arguments in input);
					(parse_named_arguments(&arguments)?, arguments.span())
				} else if lookahead.peek(syn::token::Brace) {
					let arguments;
					braced!(arguments in input);
					(parse_named_arguments(&arguments)?, arguments.span())
				} else if lookahead.peek(syn::token::Bracket) {
					let arguments;
					bracketed!(arguments in input);
					(parse_named_arguments(&arguments)?, arguments.span())
				} else {
					return Err(lookahead.error());
				};
				Ok(AtomicExpr::MacroCall(id.to_string(), arguments, Meta { span }))
			} else {
				Err(lookahead.error())
			}
		} else {
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
						let arguments = parse_simple_arguments(&arguments)?;
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
	use pretty_assertions::assert_eq;
	use quote::quote;

	use super::*;

	#[test]
	fn no_format() -> anyhow::Result<()> {
		let format = "This is a $test".to_string();
		let ast = quote! {#format};
		let result = syn::parse2::<Syntax>(ast)?;
		assert_eq!(
			result,
			Syntax {
				args: vec![(
					None,
					Expr {
						atom: AtomicExpr::LitStr(format, Meta::default()),
						suffixes: vec![],
					}
				)],
			}
		);

		Ok(())
	}

	#[test]
	fn format_self() -> anyhow::Result<()> {
		let format = "{}".to_string();
		let ast = quote! {#format, $};
		let result = syn::parse2::<Syntax>(ast)?;
		assert_eq!(
			result,
			Syntax {
				args: vec![
					(
						None,
						Expr {
							atom: AtomicExpr::LitStr(format, Meta::default()),
							suffixes: vec![],
						}
					),
					(
						None,
						Expr {
							atom: AtomicExpr::BuildInfo(Meta::default()),
							suffixes: vec![]
						}
					)
				]
			}
		);

		Ok(())
	}

	#[test]
	fn format_suffixes() -> anyhow::Result<()> {
		let format = "{}".to_string();
		let ast = quote! {#format, $.foo().7[0x0_C].foo};
		let result = syn::parse2::<Syntax>(ast)?;
		assert_eq!(
			result,
			Syntax {
				args: vec![
					(
						None,
						Expr {
							atom: AtomicExpr::LitStr(format, Meta::default()),
							suffixes: vec![],
						}
					),
					(
						None,
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
					)
				],
			}
		);

		Ok(())
	}

	#[test]
	fn format_trailing_comma() -> anyhow::Result<()> {
		let format = "3".to_string();
		let ast = quote! {#format,};
		let result = syn::parse2::<Syntax>(ast)?;
		assert_eq!(
			result,
			Syntax {
				args: vec![(
					None,
					Expr {
						atom: AtomicExpr::LitStr(format, Meta::default()),
						suffixes: vec![],
					}
				),],
			}
		);

		Ok(())
	}
}
