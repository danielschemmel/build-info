use anyhow::Result;
use num_bigint::BigInt;

use super::{
	syntax::{AtomicExpr, Expr, Suffix},
	Value, OP_ARRAY_INDEX, OP_FIELD_ACCESS, OP_TUPLE_INDEX,
};

pub(crate) trait Eval {
	fn eval(&self) -> Result<Box<dyn Value>>;
}

impl Eval for AtomicExpr {
	fn eval(&self) -> Result<Box<dyn Value>> {
		match self {
			AtomicExpr::LitBool(value, _) => Ok(Box::new(*value)),
			AtomicExpr::LitChar(value, _) => Ok(Box::new(*value)),
			AtomicExpr::LitInt(value, _) => Ok(Box::new(value.clone())),
			AtomicExpr::LitStr(value, _) => Ok(Box::new(value.clone())),
			AtomicExpr::BuildInfo(_) => Ok(Box::new(crate::deserialize_build_info())),
			AtomicExpr::Parenthesized(expr, _) => expr.eval(),
			AtomicExpr::FunctionCall(name, args, meta) => {
				let args: Result<Vec<Box<dyn Value>>> = args.iter().map(|expr| expr.eval()).collect();
				super::value::call_function(name, &args?, meta.span)
			}
			AtomicExpr::MacroCall(name, args, meta) => {
				let args: Result<Vec<_>> = args
					.iter()
					.map(|(name, expr)| Ok((name.as_ref().map(|id| id.to_string()), expr.eval()?)))
					.collect();
				super::value::call_macro(name, &args?, meta.span)
			}
		}
	}
}

impl Eval for Expr {
	fn eval(&self) -> Result<Box<dyn Value>> {
		let mut value = self.atom.eval()?;

		for suffix in &self.suffixes {
			match suffix {
				Suffix::Unwrap => {
					value = value.call("?", &[])?;
				}
				Suffix::Field(name) => {
					value = value.call(OP_FIELD_ACCESS, &[Box::new(name.clone())])?;
				}
				Suffix::TupleIndex(index) => {
					let index: BigInt = (*index).into();
					value = value.call(OP_TUPLE_INDEX, &[Box::new(index.clone())])?;
				}
				Suffix::ArrayIndex(expr) => {
					value = value.call(OP_ARRAY_INDEX, &[expr.eval()?])?;
				}
				Suffix::FunctionCall(name, args) => {
					let args = args
						.iter()
						.map(|arg| arg.eval())
						.collect::<Result<Vec<Box<dyn Value>>>>()?;
					value = value.call(name, &args)?;
				}
			}
		}

		Ok(value)
	}
}
