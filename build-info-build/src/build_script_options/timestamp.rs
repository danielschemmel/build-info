use chrono::{DateTime, TimeZone, Utc};

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
		self.timestamp = Some(Utc.timestamp_nanos(nanos));
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
		match Utc.timestamp_opt(epoch, 0) {
			chrono::LocalResult::None => panic!("Invalid SOURCE_DATE_EPOCH: {epoch}"),
			chrono::LocalResult::Single(timestamp) => timestamp,
			chrono::LocalResult::Ambiguous(min, max) => {
				panic!("Ambiguous epoch: {epoch} could refer to {min} or {max}. This should never occur for UTC!")
			}
		}
	} else {
		Utc::now()
	}
}

#[cfg(test)]
mod test {
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn get_current_timestamp() {
		let past = Utc.timestamp_opt(1591113000, 0).single().unwrap();
		let now = get_timestamp_internal(None);
		let future = Utc.timestamp_opt(32503680000, 0).single().unwrap();
		assert!(past < now);
		assert!(now < future);
	}

	#[test]
	fn get_fixed_timestamp() {
		let epoch = 1591113000;
		assert_eq!(
			get_timestamp_internal(Some(epoch.to_string())),
			Utc.timestamp_opt(epoch, 0).single().unwrap()
		);
	}
}
