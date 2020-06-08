use anyhow::Result;
use format_buf::format;
use num_bigint::BigInt;

use std::any::Any;

use super::{as_arguments_0, as_index, FormatSpecifier, Type, Value, OP_ARRAY_INDEX};

impl<T: 'static + Value + Clone> Value for Vec<T> {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			OP_ARRAY_INDEX => Ok(Box::new(self[as_index(args)].clone())),
			"len" => {
				as_arguments_0(args)?;
				Ok(Box::new(BigInt::from(self.len())))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Option
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => {
				for (i, value) in self.iter().enumerate() {
					if i > 0 {
						if i < self.len() - 1 {
							*buffer += ", ";
						} else {
							*buffer += " and ";
						}
					}
					value.format(buffer, spec);
				}
			}
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}
