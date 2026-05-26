use std::{fs, path};
use directories::ProjectDirs;

pub fn installation() -> bool {
    let Some(project_dirs) = ProjectDirs::from("org", "carla", "carla") else {
        return false;
    };

    let carla_dir = project_dirs.data_dir();
    if !fs::exists(carla_dir).unwrap_or_default() { return false; }

    let data = carla_dir.join("installation-path");
    if !fs::exists(&data).unwrap_or_default() { return false; }

    true
}

pub fn get_installation() -> String {
    let Some(project_dirs) = ProjectDirs::from("org", "carla", "carla") else {
        unreachable!()
    };

    let carla_dir = project_dirs.data_dir();
    if !fs::exists(carla_dir).unwrap_or_default() { unreachable!() }

    let data = carla_dir.join("installation-path");
    if !fs::exists(&data).unwrap_or_default() { unreachable!() }

    fs::read_to_string(data).unwrap_or_default()
}

pub fn set_installation(path: &String) {
    let Some(project_dirs) = ProjectDirs::from("org", "carla", "carla") else {
        unreachable!()
    };

    let carla_dir = project_dirs.data_dir();
    if !fs::exists(carla_dir).unwrap_or_default() {
        _ = std::fs::create_dir_all(carla_dir);
    }

    let data = carla_dir.join("installation-path");
    _ = fs::write(data, path);
}
