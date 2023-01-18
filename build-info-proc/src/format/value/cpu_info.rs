use std::any::Any;

use build_info_common::CpuInfo;
use num_bigint::BigInt;

use super::{as_arguments_0, as_field_name, FormatSpecifier, Type, Value, OP_FIELD_ACCESS};

impl Value for CpuInfo {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			OP_FIELD_ACCESS => match as_field_name(args) {
				"arch" => Ok(Box::new(self.arch.clone())),
				"pointer_width" => Ok(Box::new(BigInt::from(self.pointer_width))),
				"endianness" => Ok(Box::new(self.endianness)),
				"features" => Ok(Box::new(self.features.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::CompilerInfo
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
