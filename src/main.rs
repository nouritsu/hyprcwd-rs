mod error;

use clap::Parser;
use error::{HyprCWDError as Error, HyprCWDResult as Result};
use hyprland::data::Client;
use hyprland::shared::HyprDataActiveOptional;
use procfs::process::Process;
use std::{env, path::PathBuf, process::exit};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    default_dir: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    match active_window_cwd() {
        Ok(working_dir) => {
            println!("{}", working_dir);
        }
        Err(Error::NoActiveWindow) if args.default_dir.is_some() => unsafe {
            println!("{}", args.default_dir.unwrap_unchecked().to_string_lossy())
        },
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
}

fn active_window_cwd() -> Result<String> {
    let active_window = Client::get_active()?.ok_or(Error::NoActiveWindow)?;
    let window_pid = active_window.pid;

    let child_pid = newest_child_process(window_pid)?;

    process_cwd(child_pid).or_else(|_| home_dir())
}

fn newest_child_process(parent_pid: i32) -> Result<i32> {
    let all_processes = procfs::process::all_processes()?;

    all_processes
        .flatten()
        .flat_map(|p| p.stat())
        .filter(|p| p.ppid == parent_pid)
        .max_by_key(|p| p.starttime)
        .map_or(Ok(parent_pid), |p| Ok(p.pid))
}

fn process_cwd(pid: i32) -> Result<String> {
    let process = Process::new(pid)?;
    let cwd = process.cwd()?;

    if cwd.exists() && cwd.is_dir() {
        Ok(cwd.to_string_lossy().to_string())
    } else {
        home_dir()
    }
}

fn home_dir() -> Result<String> {
    env::var("HOME").map_err(Error::EnvVarError)
}
