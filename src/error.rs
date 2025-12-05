use hyprland::shared::HyprError;
use procfs::ProcError;
use std::env::VarError;
use thiserror::Error;

pub type HyprCWDResult<T> = Result<T, HyprCWDError>;

#[derive(Error, Debug)]
pub enum HyprCWDError {
    #[error("error(hyprland): {0}")]
    HyprlandError(#[from] HyprError),
    #[error("error(procfs): {0}")]
    ProcfsError(#[from] ProcError),
    #[error("error(env): {0}")]
    EnvVarError(#[from] VarError),
    #[error("error(active_window): no active window found, default not specified")]
    NoActiveWindow,
}
