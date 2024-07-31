use std::{io::{BufRead, BufReader} ,process::{Child, Command, Stdio}};

use anyhow::{anyhow, Ok};
use regex::Regex;
use rand::Rng;

use crate::{endgroup, group};
use crate::utils::get_command_line_from_command;

/// Run a process
pub fn run_process(name: &str, args: &Vec<&str>, error_msg: &str, quiet: bool) -> anyhow::Result<()> {
    let joined_args = args.join(" ");
    if !quiet {
        info!("Command line: {} {}", name, &joined_args);
    }
    let status = Command::new(name)
        .args(args)
        .status()
        .map_err(|e| anyhow!("Failed to execute {} {}: {}", name, args.first().unwrap(), e))?;
    if !status.success() {
        return Err(anyhow!("{}", error_msg));
    }
    Ok(())
}

/// Run a process for workspace
/// regexp must have one capture group if defined
pub fn run_process_for_workspace<'a>(
    name: &str,
    mut args: Vec<&'a str>,
    excluded: &'a Vec<String>,
    error_msg: &str,
    group_regexp: Option<&str>,
) -> anyhow::Result<()> {
    let re: Option<Regex> = group_regexp.map(|r| Regex::new(r).unwrap());
    excluded
        .iter()
        .for_each(|ex| args.extend(["--exclude", &ex]));
    info!("Command line: cargo {}", args.join(" "));
    let mut child = Command::new(name)
        .args(&args)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!(format!("Failed to start {} {}: {}", name, args.first().unwrap(), e)))?;

    let mut close_group = false;
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        reader.lines().for_each(|line| {
            line.ok().map(|line| {
                if let Some(rx) = &re {
                    rx.captures(&line).map(|caps| {
                        let crate_name = &caps[1];
                        if close_group {
                            endgroup!();
                        }
                        group!("Unit Tests: {}", crate_name);
                    });
                }
                eprintln!("{}", line);
                close_group = true;
            });
        });
    }
    if close_group {
        endgroup!();
    }
    let status = child.wait().expect("Should be able to wait for the process to finish.");
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("{}", error_msg))
    }
}

/// Run a process command for a package
pub fn run_process_for_package(
    name: &str,
    package: &String,
    args: &Vec<&str>,
    excluded: &Vec<String>,
    only: &Vec<String>,
    error_msg: &str,
    ignore_log: Option<&str>,
    ignore_msg: Option<&str>,
) -> anyhow::Result<()> {
    if excluded.contains(package) || (!only.is_empty() && !only.contains(package)) {
        info!("Skip '{}' because it has been excluded!", package);
        return Ok(());
    }
    let joined_args = args.join(" ");
    info!("Command line: cargo {}", &joined_args);
    let output = Command::new("cargo")
        .args(args)
        .output()
        .map_err(|e| anyhow!("Failed to execute process for '{}': {}", name, e))?;

    if output.status.success() {
        return Ok(());
    } else if let Some(log) = ignore_log {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains(log) {
            if let Some(msg) = ignore_msg {
                warn!("{}", msg);
            }
            endgroup!();
            return Ok(());
        }
    }
    return Err(anyhow!("{}", error_msg));
}

/// Spawn a process from passed command
pub fn run_process_command(command: &mut Command, error: &str) -> anyhow::Result<()> {
    // Handle cargo child process
    let command_line = get_command_line_from_command(command);
    info!("{command_line}\n");
    let process = command.spawn().expect(error);
    let error = format!(
        "{} process should run flawlessly",
        command.get_program().to_str().unwrap()
    );
    handle_child_process(process, &error)
}

/// Run a command
pub fn run_command(
    command: &str,
    args: &[&str],
    command_error: &str,
    child_error: &str,
) -> anyhow::Result<()> {
    // Format command
    info!("{command} {}\n\n", args.join(" "));

    // Run command as child process
    let command = Command::new(command)
        .args(args)
        .stdout(Stdio::inherit()) // Send stdout directly to terminal
        .stderr(Stdio::inherit()) // Send stderr directly to terminal
        .spawn()
        .expect(command_error);

    // Handle command child process
    handle_child_process(command, child_error)
}

/// Handle child process
pub fn handle_child_process(mut child: Child, error: &str) -> anyhow::Result<()> {
    // Wait for the child process to finish
    let status = child.wait().expect(error);

    // If exit status is not a success, terminate the process with an error
    if !status.success() {
        // Use the exit code associated to a command to terminate the process,
        // if any exit code had been found, use the default value 1
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

/// Return a random port between 3000 and 9999
pub fn random_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(3000..=9999)
}
