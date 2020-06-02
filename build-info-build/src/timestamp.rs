use crate::{DateTime, Utc};

impl crate::BuildScriptOptions {
	/// Set the build timestamp by hand.
	///
	/// This is mostly important for reproducible builds using only cargo. If possible, consider setting the environment
	/// variable [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/specs/source-date-epoch/) instead, which does not
	/// require any setup.
	pub fn build_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
		self.timestamp = Some(timestamp);
		self
	}

	/// Set the build timestamp by hand as nanosecond-precise UNIX timestamp.
	///
	/// This is mostly important for reproducible builds using only cargo. If possible, consider setting the environment
	/// variable [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/specs/source-date-epoch/) instead, which does not
	/// require any setup.
	pub fn build_timestamp_as_nanos(mut self, nanos: i64) -> Self {
		self.timestamp = Some(build_info_common::nanos_to_utc(nanos));
		self
	}
}

pub(crate) fn get_timestamp() -> DateTime<Utc> {
	get_timestamp_internal(std::env::var("SOURCE_DATE_EPOCH").ok())
}

fn get_timestamp_internal(epoch: Option<String>) -> DateTime<Utc> {
	// https://reproducible-builds.org/specs/source-date-epoch/
	if let Some(epoch) = epoch {
		let epoch: i64 = epoch.parse().expect("Could not parse SOURCE_DATE_EPOCH");
		build_info_common::epoch_to_utc(epoch)
	} else {
		build_info_common::Utc::now()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn get_current_timestamp() {
		let past = build_info_common::epoch_to_utc(1591113000);
		let now = get_timestamp_internal(None);
		let future = build_info_common::epoch_to_utc(32503680000);
		assert!(past < now);
		assert!(now < future);
	}

	#[test]
	fn get_fixed_timestamp() {
		let epoch = 1591113000;
		assert_eq!(
			get_timestamp_internal(Some(epoch.to_string())),
			build_info_common::epoch_to_utc(epoch)
		);
	}
}
