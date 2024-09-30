use std::fs;

use crate::{opt, CONFIG_FILE_NAME};
use colored::Colorize;
use isograph_config::CompilerConfig;

/// Creates a config file based on the command parameters provided
pub fn create_config_file(create_config_file_command: opt::CreateConfigFileCommand) {
    let config_location = create_config_file_command.config.clone();
    let config = populate_config(create_config_file_command);

    let config_contents = serde_json::to_string_pretty(&config).unwrap();

    fs::write(config_location.join(CONFIG_FILE_NAME), config_contents).unwrap();
    println!(
        "{}",
        format!(
            "Successfully created config file at {:?}",
            config_location.join(CONFIG_FILE_NAME)
        )
        .bright_green()
    );
}

/// Prepares a proper `CompilerConfig` based on the command parameters provided
fn populate_config(create_config_file_command: opt::CreateConfigFileCommand) -> CompilerConfig {
    let project_root = create_config_file_command.project_root;

    CompilerConfig {
        project_root: project_root.clone(),
        artifact_directory: create_config_file_command
            .artifact_directory
            .unwrap_or(project_root),
        schema: create_config_file_command.schema,
        schema_extensions: create_config_file_command
            .schema_extensions
            .unwrap_or_default(),
        options: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use opt::Opt;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn config_location_default_is_root() {
        let config = Opt::parse_from(vec![
            "isograph",
            "create-config",
            "--schema",
            "./src/schema.graphql",
        ]);

        if let Some(opt::Command::CreateConfig(create_config_file_command)) = config.command {
            assert_eq!(create_config_file_command.config, PathBuf::from("./"));
        }
    }

    #[test]
    fn schema_is_required() {
        assert!(Opt::try_parse_from(vec!["isograph", "create-config"]).is_err());
    }

    #[test]
    fn project_root_default_is_correct() {
        let config = Opt::parse_from(vec![
            "isograph",
            "create-config",
            "--schema",
            "./src/schema.graphql",
        ]);

        if let Some(opt::Command::CreateConfig(create_config_file_command)) = config.command {
            let compiler_config = populate_config(create_config_file_command);

            assert_eq!(
                compiler_config.project_root,
                PathBuf::from("./src/components")
            );
        }
    }

    #[test]
    fn artifact_directory_is_project_root_if_not_provided() {
        let config = Opt::parse_from(vec![
            "isograph",
            "create-config",
            "--schema",
            "./src/schema.graphql",
            "--project-root",
            "./src/components",
        ]);

        if let Some(opt::Command::CreateConfig(create_config_file_command)) = config.command {
            let compiler_config = populate_config(create_config_file_command);

            assert_eq!(
                compiler_config.artifact_directory,
                compiler_config.project_root
            );
        }
    }
}
