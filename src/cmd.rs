use std::{
	env,
	error::Error,
	fs,
	path::PathBuf,
};

use toml_edit::{
	Document,
	Item,
};

use crate::{
	app,
	Version,
	VersionField,
};

enum Command {
	Set(Version),
	Get,
	Bump(VersionField),
}

pub struct Cmd {
	cmd: Command,
	toml_path: PathBuf,
}

impl Cmd {
	pub fn from_args() -> Result<Self, Box<dyn Error>> {
		let args = env::args().skip(1).collect::<Vec<_>>();
		if args.is_empty() {
			return Err("this command should be invoked from cargo (cargo ver)".into());
		}
		let matches = app::new().get_matches_from(args);
		let toml_path = match matches.value_of("toml").map(PathBuf::from) {
			Some(p) => p,
			None => find_toml()?,
		};

		Ok(match matches.subcommand_name() {
			None => Self {
				toml_path,
				cmd: Command::Get,
			},
			Some("bump") => {
				let m = matches.subcommand_matches("bump").unwrap();
				let field = match &m.value_of("field").unwrap().to_lowercase()[..] {
					"p" | "pat" | "patch" => VersionField::Patch,
					"min" | "minor" => VersionField::Minor,
					"maj" | "major" => VersionField::Major,
					s => panic!("unhandled field in match expression: {:?}", s),
				};
				Self {
					toml_path,
					cmd: Command::Bump(field),
				}
			}
			Some("set") => {
				let m = matches.subcommand_matches("set").unwrap();
				let val = m.value_of("version").unwrap().parse::<Version>()?;
				Self {
					toml_path,
					cmd: Command::Set(val),
				}
			}
			s => panic!("unhandled match case for subcommands: {:?}", s),
		})
	}

	pub fn run(self) -> Result<(), Box<dyn Error>> {
		let Self { toml_path, cmd } = self;

		let data = fs::read_to_string(&toml_path)?;

		let mut man = data.parse::<Document>()?;
		let old_version_str = match &mut man["package"] {
			Item::None => {
				return Err(format!(
					"the manifest at {} does not contain a [package] section",
					toml_path.display()
				)
				.into())
			}
			Item::Table(pkg) => match pkg.entry("version") {
				Item::None => return Err("the crate manifest is missing the version field".into()),
				Item::Value(v) if v.is_str() => v,
				_ => {
					return Err(
						"the crate manifests `[package].version] field must be a string".into(),
					)
				}
			},
			_ => {
				return Err(format!(
					"unrecognized manifest at {}: the `package` must be a table",
					toml_path.display()
				)
				.into())
			}
		};

		let old_version = old_version_str
			.as_str()
			.unwrap()
			.parse::<Version>()
			.map_err(|e| format!("the crate version field is invalid: {}", e))?;
		let new_version = match cmd {
			Command::Get => {
				println!("{}", &old_version);
				return Ok(());
			}
			Command::Set(new_version) => new_version,
			Command::Bump(field) => {
				let mut new_version = old_version.clone();
				match field {
					VersionField::Patch => new_version.bump_patch(),
					VersionField::Minor => new_version.bump_minor(),
					VersionField::Major => new_version.bump_major(),
				};
				new_version
			}
		};

		if new_version.eq(&old_version) {
			println!(
				"the new version is the same as the old one ({}); no action taken",
				&old_version
			);
			return Ok(());
		}
		let val = toml_edit::value(new_version.to_string());
		let decor = old_version_str.decor();
		let val = toml_edit::decorated(
			val.as_value().unwrap().clone(),
			decor.prefix(),
			decor.suffix(),
		);
		*old_version_str = val;

		let data = man.to_string();
		fs::write(&toml_path, data.as_bytes())?;
		println!("updated version: {} -> {}", &old_version, &new_version);
		Ok(())
	}
}

fn find_toml() -> Result<PathBuf, Box<dyn Error>> {
	let mut dir = env::current_dir()?;
	loop {
		dir.push("Cargo.toml");
		if dir.exists() {
			return Ok(dir);
		}
		dir.pop();
		if !dir.pop() {
			return Err("could not locate a `Cargo.toml` in any of the current directory or the parent directories".into());
		}
	}
}
