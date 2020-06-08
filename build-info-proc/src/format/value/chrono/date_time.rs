use format_buf::format;

use std::any::Any;

use build_info_common::chrono::{DateTime, Utc};

use super::super::{FormatSpecifier, Type, Value};

impl Value for DateTime<Utc> {
	fn get_type(&self) -> Type {
		Type::DateTimeUtc
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self.format("%Y-%m-%d %H:%M:%SZ")),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}
