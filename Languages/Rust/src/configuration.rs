use std::{env, fs, path::PathBuf};

use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{
	identity::{APPLICATION_NAME, AUTHOR_DOMAIN, AUTHOR_TLD},
	logging::LoggingConfiguration,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Configuration {
	pub logging: LoggingConfiguration,
}

impl Configuration {
	pub fn load() -> Self {
		const CONFIGURATION_FILE_NAME: &str = "config.toml";

		let project_dirs = ProjectDirs::from(AUTHOR_TLD, AUTHOR_DOMAIN, APPLICATION_NAME).unwrap();

		let direct_configuration_path: PathBuf = env::current_dir()
			.unwrap()
			.join("config")
			.join(CONFIGURATION_FILE_NAME);

		let system_configuration_path: PathBuf =
			["/etc", APPLICATION_NAME, CONFIGURATION_FILE_NAME]
				.iter()
				.collect();

		let local_configuration_path: PathBuf = project_dirs
			.config_dir()
			.join(CONFIGURATION_FILE_NAME)
			.to_path_buf();

		if !fs::exists(&direct_configuration_path).unwrap()
			&& !fs::exists(&system_configuration_path).unwrap()
			&& !fs::exists(&local_configuration_path).unwrap()
		{
			panic!(
				"Unable to find a configuration at:\n\t{}\t\n{}\t\n{}",
				direct_configuration_path.to_str().unwrap(),
				system_configuration_path.to_str().unwrap(),
				local_configuration_path.to_str().unwrap(),
			);
		}

		let mut config_builder = Config::builder();

		config_builder = LoggingConfiguration::new(config_builder, "logging");

		let config = config_builder
			.add_source(
				File::with_name(direct_configuration_path.to_str().unwrap()).required(false),
			)
			.add_source(
				File::with_name(system_configuration_path.to_str().unwrap()).required(false),
			)
			.add_source(File::with_name(local_configuration_path.to_str().unwrap()).required(false))
			.build()
			.unwrap();

		config.try_deserialize::<Self>().unwrap()
	}
}
