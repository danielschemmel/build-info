build_info::build_info!(fn version);

fn print_crate_info(ci: &build_info::CrateInfo, self_indent: &str, nest_indent: &str) {
	println!(
		"{}{} v{} {}",
		self_indent,
		ci.name,
		ci.version,
		ci.enabled_features.join(", ")
	);

	let nested_self = format!("{}├──", nest_indent);
	let nested_self_last = format!("{}└──", nest_indent);
	let nested_nest = format!("{}│  ", nest_indent);
	let nested_nest_last = format!("{}   ", nest_indent);
	for (i, dep) in ci.dependencies.iter().enumerate() {
		if i + 1 < ci.dependencies.len() {
			print_crate_info(dep, &nested_self, &nested_nest);
		} else {
			print_crate_info(dep, &nested_self_last, &nested_nest_last);
		}
	}
}

fn main() {
	print_crate_info(&version().crate_info, "", "");
}
