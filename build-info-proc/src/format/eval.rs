use anyhow::Result;
use num_bigint::BigInt;

use super::syntax::{AtomicExpr, Expr, Suffix};
use super::Value;

pub(crate) trait Eval {
	fn eval(&self) -> Result<Box<dyn Value>>;
}

impl Eval for AtomicExpr {
	fn eval(&self) -> Result<Box<dyn Value>> {
		match self {
			AtomicExpr::LitInt(value, _) => Ok(Box::new(value.clone())),
			AtomicExpr::LitStr(value, _) => Ok(Box::new(value.clone())),
			AtomicExpr::BuildInfo(_) => Ok(Box::new(crate::deserialize_build_info())),
			AtomicExpr::Parenthesized(expr, _) => expr.eval(),
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
					value = value.call("!field", &[name])?;
				}
				Suffix::TupleIndex(index) => {
					let index: BigInt = (*index).into();
					value = value.call("!tuple_index", &[&index])?;
				}
				Suffix::ArrayIndex(expr) => {
					let index = expr.eval()?;
					value = value.call("::std::ops::Index::index", &[&*index])?;
				}
				Suffix::FunctionCall(name, args) => {
					let args = args
						.iter()
						.map(|arg| arg.eval())
						.collect::<Result<Vec<Box<dyn Value>>>>()?;
					let args: Vec<&dyn Value> = args.iter().map(|arg| &**arg).collect();
					value = value.call(name, &args[..])?;
				}
			}
		}

		Ok(value)
	}
}
