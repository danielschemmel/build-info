use build_info_common::{CpuInfo, Endianness, TargetInfo};

pub(crate) fn get_info() -> TargetInfo {
	TargetInfo {
		triple: std::env::var("TARGET").unwrap_or_else(|_| "UNKNOWN".to_string()),
		family: std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_else(|_| "UNKNOWN".to_string()),
		os: std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "UNKNOWN".to_string()),
		cpu: CpuInfo {
			arch: std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "UNKNOWN".to_string()),
			pointer_width: std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
				.expect("Could not read `CARGO_CFG_TARGET_POINTER_WIDTH`")
				.parse()
				.expect("Could not parse the target pointer width from `CARGO_CFG_TARGET_POINTER_WIDTH`"),
			endianness: match std::env::var("CARGO_CFG_TARGET_ENDIAN") {
				Ok(val) => match val.as_str() {
					"little" => Endianness::Little,
					"big" => Endianness::Big,
					_ => panic!("Unknown endianness: {val:?}"),
				},
				Err(err) => panic!("Could not read `CARGO_CFG_TARGET_ENDIAN`: {err}"),
			},
			features: std::env::var("CARGO_CFG_TARGET_FEATURE")
				.unwrap_or_default()
				.split(',')
				.map(|s| s.to_owned())
				.collect(),
		},
	}
}
