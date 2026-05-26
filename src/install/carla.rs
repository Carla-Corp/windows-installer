use druid::{ExtEventSink, Target};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use std::thread;
use std::time::Duration;

use crate::install::{animate_progress, finish};

use crate::SET_PROGRESS;

pub fn install(sink: ExtEventSink, installation: &str) {
    let progress = Arc::new(Mutex::new(0.0));

    let running = Arc::new(AtomicBool::new(true));
    let added = Arc::new(Mutex::new(0.0));

    animate_progress(
        sink.clone(),
        Arc::clone(&progress),
        0.1,
        Arc::clone(&running),
        Arc::clone(&added),
    );

    let url = "https://github.com/lucasFelixSilveira/carla/archive/refs/heads/main.zip";

    let mut response = reqwest::blocking::get(url).unwrap();
    let mut file = File::create("carla.zip").unwrap();

    let mut buffer = [0; 8192];

    loop {
        let n = response.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n]).unwrap();
    }

    running.store(false, Ordering::Relaxed);

    {
        let mut p = progress.lock().unwrap();
        (*p) += 0.1 - *added.lock().unwrap();
        let _ = sink.submit_command(SET_PROGRESS, *p, Target::Auto);
        *added.lock().unwrap() = 0.0;
    }

    let running = Arc::new(AtomicBool::new(true));

    animate_progress(
        sink.clone(),
        Arc::clone(&progress),
        0.1,
        Arc::clone(&running),
        Arc::clone(&added),
    );

    let temp_path = Path::new("temp");
    if !temp_path.exists() {
        fs::create_dir_all(temp_path).unwrap();
    }

    let zip_file = File::open("carla.zip").unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = temp_path.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).unwrap();
                }
            }

            let mut outfile = File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    running.store(false, Ordering::Relaxed);

    {
        let mut p = progress.lock().unwrap();
        (*p) += 0.1 - *added.lock().unwrap();
        let _ = sink.submit_command(SET_PROGRESS, *p, Target::Auto);
        *added.lock().unwrap() = 0.0;
    }

    use std::process::Command;

    let running = Arc::new(AtomicBool::new(true));

    animate_progress(
        sink.clone(),
        Arc::clone(&progress),
        0.2,
        Arc::clone(&running),
        Arc::clone(&added),
    );

    thread::sleep(Duration::from_millis(400));
    running.store(false, Ordering::Relaxed);

    let status = Command::new("g++")
        .current_dir("temp/carla-main/compiler")
        .args([
            "-std=c++17",
            "-g",
            "-O0",
            "-fPIC",
            "-fpermissive",
            "-fexceptions",
            "main.cpp",
            "-o",
            "../build/carla",
            "-I.",
            "-L./libs/x86_64-windows",
            "-leva"
        ])
        .status()
        .expect("failed to execute g++");

    if !status.success() {
        eprintln!("Compilation failed");
        return;
    }

    running.store(false, Ordering::Relaxed);

    {
        let mut p = progress.lock().unwrap();
        (*p) += 0.2 - *added.lock().unwrap();
        let _ = sink.submit_command(SET_PROGRESS, *p, Target::Auto);
        *added.lock().unwrap() = 0.0;
    }

    finish::finish(sink, installation);
}
