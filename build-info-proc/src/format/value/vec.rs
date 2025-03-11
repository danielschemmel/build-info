use std::any::Any;

use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use proc_macro_error2::abort_call_site;

use super::{FormatSpecifier, OP_ARRAY_INDEX, Type, Value, as_arguments_0, as_index, as_simple_arguments_1};

impl<T: 'static + Value + Clone> Value for Vec<T> {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			"get" => {
				let (index,) = as_simple_arguments_1::<BigInt>(args)?;
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => {
				for (i, value) in self.iter().enumerate() {
					if i > 0 {
						if i < self.len() - 1 {
							*buffer += ", ";
						} else {
							#[cfg(feature = "oxford-comma")]
							{
								if i > 1 {
									*buffer += ","
								}
							}
							*buffer += " and ";
						}
					}
					value.format(buffer, spec);
				}
			}
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
	fn format_vec0() {
		let value: &dyn Value = &vec![] as &Vec<String>;
		let mut result = String::new();
		value.format(&mut result, FormatSpecifier::Default);
		assert_eq!(&result, "");
	}

	#[test]
	fn format_vec1() {
		let value: &dyn Value = &vec!["abc".to_string()];
		let mut result = String::new();
		value.format(&mut result, FormatSpecifier::Default);
		assert_eq!(&result, "abc");
	}

	#[test]
	fn format_vec2() {
		let value: &dyn Value = &vec!["ab".to_string(), "cd".to_string()];
		let mut result = String::new();
		value.format(&mut result, FormatSpecifier::Default);
		assert_eq!(&result, "ab and cd");
	}

	#[test]
	fn format_vec3() {
		let value: &dyn Value = &vec!["ab".to_string(), "cd".to_string(), "ef".to_string()];
		let mut result = String::new();
		value.format(&mut result, FormatSpecifier::Default);

		#[cfg(feature = "oxford-comma")]
		assert_eq!(&result, "ab, cd, and ef");

		#[cfg(not(feature = "oxford-comma"))]
		assert_eq!(&result, "ab, cd and ef");
	}
}
