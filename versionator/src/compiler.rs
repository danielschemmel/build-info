use rustc_version::{version_meta, Channel};

use versionator::{CompilerChannel, CompilerVersion};

pub fn get_info() -> CompilerVersion {
	let version = version_meta().unwrap();

	let channel = match version.channel {
		Channel::Stable => CompilerChannel::Stable,
		Channel::Beta => CompilerChannel::Beta,
		Channel::Nightly => CompilerChannel::Nightly,
		Channel::Dev => CompilerChannel::Dev,
	};

	CompilerVersion {
		version: version.semver,
		commit_hash: version.commit_hash,
		commit_date: version.commit_date,
		channel: channel,
		host_triple: version.host,
		target_triple: std::env::var("TARGET").unwrap_or("UNKNOWN".to_string()),
	}
}
