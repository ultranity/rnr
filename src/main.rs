//! # RnR
//! *RnR* is a command-line tool to rename multiple files and directories that supports regex
//! expressions.
//!
extern crate ansi_term;
extern crate any_ascii;
extern crate chrono;
extern crate difference;
extern crate path_abs;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate walkdir;

extern crate clap;
extern crate serde_derive;

use crate::renamer::Renamer;
use std::io::Write;

mod cli;
mod config;
mod dumpfile;
mod editor;
mod error;
mod fileutils;
mod output;
mod renamer;
mod solver;

fn main() {
    // Read arguments
    let config = match config::Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    if !config.force {
        let info = &config.printer.colors.info;
        config
            .printer
            .print(&format!("{}", info.paint("This is a DRY-RUN")));
    }

    // Configure renamer
    let renamer = match Renamer::new(&config) {
        Ok(renamer) => renamer,
        Err(err) => {
            config.printer.print_error(&err);
            std::process::exit(1);
        }
    };

    // Generate operations
    let (operations, deletions) = match renamer.process() {
        Ok(result) => result,
        Err(err) => {
            config.printer.print_error(&err);
            std::process::exit(1);
        }
    };

    let interactive = matches!(
        config.run_mode,
        config::RunMode::Editor {
            interactive: true,
            ..
        }
    );

    if interactive {
        if let Err(err) = run_batch(&renamer, operations.clone(), deletions.clone(), false) {
            config.printer.print_error(&err);
            std::process::exit(1);
        }

        if !confirm_apply() {
            config.printer.print("Aborted. Changes were not applied.");
            return;
        }
        config.printer.print("Applying changes...");
    }

    let force = if interactive { true } else { config.force };
    if let Err(err) = run_batch(&renamer, operations, deletions, force) {
        config.printer.print_error(&err);
        std::process::exit(1);
    }
}

fn run_batch(
    renamer: &Renamer,
    operations: solver::Operations,
    deletions: Vec<std::path::PathBuf>,
    force: bool,
) -> error::Result<()> {
    // Batch rename operations. If some rename targets are also in the deletion list
    // (editor --delete flow), those paths are deleted right before the conflicting rename.
    let deletions = renamer.batch_rename(operations, deletions, force)?;
    // Batch delete remaining operations (only populated for editor mode with --delete)
    renamer.batch_delete(deletions, force)?;
    Ok(())
}

fn confirm_apply() -> bool {
    print!("Apply these changes? [y/N]: ");
    let _ = std::io::stdout().flush();
    let mut answer = String::new();
    if std::io::stdin().read_line(&mut answer).is_err() {
        return false;
    }
    matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}
