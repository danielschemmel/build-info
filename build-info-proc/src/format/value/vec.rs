use anyhow::Result;
use format_buf::format;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use proc_macro_error::abort_call_site;

use std::any::Any;

use super::{as_arguments_0, as_arguments_1, as_index, FormatSpecifier, Type, Value, OP_ARRAY_INDEX};

impl<T: 'static + Value + Clone> Value for Vec<T> {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> Result<Box<dyn Value>> {
		match func {
			"get" => {
				let (index,) = as_arguments_1::<BigInt>(args)?;
				Ok(Box::new(index.to_usize().and_then(|index| self.get(index).cloned())))
			}
			"is_empty" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.is_empty()))
			}
			"len" => {
				as_arguments_0(args)?;
				Ok(Box::new(BigInt::from(self.len())))
			}
			OP_ARRAY_INDEX => {
				let index = as_index(args);
				let value = self
					.get(index)
					.unwrap_or_else(|| {
						abort_call_site!(
							"Index out of bounds: the len is {} but the index is {}",
							self.len(),
							index
						)
					})
					.clone();
				Ok(Box::new(value))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Vec
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
