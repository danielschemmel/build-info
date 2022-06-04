use anyhow::Result;
use num_bigint::BigInt;

use std::any::Any;

use super::{as_arguments_0, FormatSpecifier, Type, Value};

impl Value for String {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> Result<Box<dyn Value>> {
		match func {
			"is_empty" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.is_empty()))
			}
			"len" => {
				as_arguments_0(args)?;
				Ok(Box::new(BigInt::from(self.len())))
			}
			"to_lowercase" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_lowercase()))
			}
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			"to_uppercase" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_uppercase()))
			}
			"trim" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.trim().to_string()))
			}
			"trim_end" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.trim_end().to_string()))
			}
			"trim_start" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.trim_start().to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::String
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => *buffer += self,
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn format_default() {
		let mut buff = String::new();
		Value::format(&"abc".to_string(), &mut buff, FormatSpecifier::Default);
		assert_eq!(buff, "abc");

		buff.clear();
		Value::format(&"\0!".to_string(), &mut buff, FormatSpecifier::Default);
		assert_eq!(buff, format!("{}", "\0!"));
	}

	#[test]
	fn format_debug() {
		let mut buff = String::new();
		Value::format(&"abc".to_string(), &mut buff, FormatSpecifier::Debug);
		assert_eq!(buff, "\"abc\"");

		buff.clear();
		Value::format(&"\0!".to_string(), &mut buff, FormatSpecifier::Debug);
		assert_eq!(buff, format!("{:?}", "\0!"));
	}

	#[test]
	fn format_debug_alt() {
		let mut buff = String::new();
		Value::format(&"abc".to_string(), &mut buff, FormatSpecifier::DebugAlt);
		assert_eq!(buff, "\"abc\"");

		buff.clear();
		Value::format(&"\0!".to_string(), &mut buff, FormatSpecifier::DebugAlt);
		assert_eq!(buff, format!("{:#?}", "\0!"));
	}
}
