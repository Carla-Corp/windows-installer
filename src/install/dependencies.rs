use crate::AppState;
use std::process::Command;

pub fn check() -> bool {
    let output = Command::new("g++").arg("--version").output();

    let Ok(result) = output else {
        return false;
    };

    result.status.success()
}
