use std::any::Any;

use super::{as_arguments_0, FormatSpecifier, Type, Value};

impl Value for bool {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Bool
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => write!(buffer, "{self}").unwrap(),
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}

#[cfg(test)]
mod test {
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn format_default() {
		let mut buff = String::new();
		Value::format(&true, &mut buff, FormatSpecifier::Default);
		assert_eq!(buff, "true");

		buff.clear();
		Value::format(&false, &mut buff, FormatSpecifier::Default);
		assert_eq!(buff, "false");
	}

	#[test]
	fn format_debug() {
		let mut buff = String::new();
		Value::format(&true, &mut buff, FormatSpecifier::Debug);
		assert_eq!(buff, "true");

		buff.clear();
		Value::format(&false, &mut buff, FormatSpecifier::Debug);
		assert_eq!(buff, "false");
	}

	#[test]
	fn format_debug_alt() {
		let mut buff = String::new();
		Value::format(&true, &mut buff, FormatSpecifier::DebugAlt);
		assert_eq!(buff, "true");

		buff.clear();
		Value::format(&false, &mut buff, FormatSpecifier::DebugAlt);
		assert_eq!(buff, "false");
	}
}
