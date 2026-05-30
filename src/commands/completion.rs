use anyhow::Result;
use clap::CommandFactory;
use clap_complete::aot::{Shell, generate};

use crate::Cli;

pub fn execute_completion(shell: &str) -> Result<i32> {
    let shell: Shell = shell.parse().map_err(|_| {
        anyhow::anyhow!("Invalid shell: {shell}. Supported shells: bash, zsh, fish, powershell")
    })?;

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "cora", &mut std::io::stdout());

    Ok(0)
}
