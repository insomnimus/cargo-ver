use std::{
	fmt,
	str::FromStr,
};

pub(crate) mod app;
pub mod cmd;

#[derive(Clone)]
pub enum VersionField {
	Major,
	Minor,
	Patch,
}

#[derive(Clone, PartialEq)]
pub struct Version {
	major: u32,
	minor: u32,
	patch: u32,
	tag: Option<String>,
}

impl Version {
	pub fn bump_major(&mut self) {
		self.major += 1;
		self.minor = 0;
		self.patch = 0;
	}

	pub fn bump_minor(&mut self) {
		self.minor += 1;
		self.patch = 0;
	}

	pub fn bump_patch(&mut self) {
		self.patch += 1;
	}
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{major}.{minor}.{patch}{tag}",
			major = self.major,
			minor = self.minor,
			patch = self.patch,
			tag = self.tag.as_deref().unwrap_or("")
		)
	}
}

impl FromStr for Version {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, &'static str> {
		let mut fields = s.splitn(4, '.');
		let major = fields
			.next()
			.ok_or("the `major` field is missing")?
			.parse::<u32>()
			.map_err(|_| "the `major` field must be a non-negative integer")?;

		let minor = fields
			.next()
			.ok_or("the `minor` field is missing")?
			.parse::<u32>()
			.map_err(|_| "the `minor` field must be a non-negative integer")?;

		let patch = fields
			.next()
			.ok_or("the `patch` field is missing")?
			.parse::<u32>()
			.map_err(|_| "the `patch` field must be a non-negative integer")?;

		let tag = fields.next().map(String::from);

		Ok(Self {
			major,
			minor,
			patch,
			tag,
		})
	}
}
