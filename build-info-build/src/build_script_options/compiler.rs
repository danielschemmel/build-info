use build_info_common::{chrono::NaiveDate, semver::Version, CompilerChannel, CompilerInfo};
use rustc_version::{version_meta, Channel};

pub(crate) fn get_info() -> CompilerInfo {
	let rustc_version = version_meta().unwrap();

	// By serializing and reparsing the version, we break the version-lock between semver as provided
	// by rustc_version and semver as provided and used by this crate.
	let version = Version::parse(&rustc_version.semver.to_string()).unwrap();

	let channel = match rustc_version.channel {
		Channel::Stable => CompilerChannel::Stable,
		Channel::Beta => CompilerChannel::Beta,
		Channel::Nightly => CompilerChannel::Nightly,
		Channel::Dev => CompilerChannel::Dev,
	};

	let commit_date = rustc_version
		.commit_date
		.and_then(|date| NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok());

	CompilerInfo {
		version,
		commit_id: rustc_version.commit_hash,
		commit_date,
		channel,
		host_triple: rustc_version.host,
		target_triple: std::env::var("TARGET").unwrap_or_else(|_| "UNKNOWN".to_string()),
	}
}
