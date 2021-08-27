use clap::{
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn new() -> App<'static> {
	App::new("cargo-ver")
		.about("Manage cargo crate versions.")
		.version(crate_version!())
		.global_setting(AppSettings::DisableVersionForSubcommands)
		.global_setting(AppSettings::UnifiedHelpMessage)
		.subcommand(cmd_bump())
		.subcommand(cmd_set())
		.arg(
			Arg::new("toml")
				.about("Manually set the Cargo.toml path.")
				.short('t')
				.long("toml")
				.takes_value(true),
		)
}

fn cmd_set() -> App<'static> {
	App::new("set")
		.about("Set the crate version.")
		.visible_alias("s")
		.arg(
			Arg::new("version")
				.about("A semver compatible version.")
				.required(true)
				.forbid_empty_values(true),
		)
}

fn cmd_bump() -> App<'static> {
	let app = App::new("bump")
		.about("Bump a part of the current version.")
		.visible_alias("b");

	let field = Arg::new("field")
		.about("The part to bump.")
		.long_about(
			"The part to bump.
  One of the following:
    - major [aliases: maj]
    - minor [aliases: min]
    - patch [aliases: p, pat]",
		)
		.required(true)
		.possible_values(&["patch", "minor", "major", "pat", "p", "min", "maj"])
		.forbid_empty_values(true)
		.case_insensitive(true)
		.hide_possible_values(true)
		.value_name("version field");

	app.arg(field)
}
