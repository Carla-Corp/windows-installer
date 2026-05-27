use std::{env::{self, consts::{DLL_PREFIX, DLL_SUFFIX}}, fs::{self, File}, io::{BufReader, BufWriter, Cursor, Read}, path::Path, sync::{Arc, Mutex, mpsc}};
use notify_rust::Notification;
use druid::{EventCtx, Target};
use std::process::Command;
use zip::ZipArchive;

use crate::{GET_ALERT, SET_PROGRESS, check, screens::AppState};

#[cfg(target_os = "windows")]
const BINARY_SUFFIX: &'static str = ".exe";

#[cfg(not(target_os = "windows"))]
const BINARY_SUFFIX: &'static str = "";

mod cdn;

fn run(command: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", command])
            .status()
            .unwrap();
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("sh")
            .args(["-c", command])
            .status()
            .unwrap();
    }
}

const CDN: &'static str = "https://carla-cdn.vercel.app/";

pub fn install(ctx: &mut EventCtx<'_, '_>, state: &mut AppState) {
    let temp = env::temp_dir();
    let libraries = temp.join("carla-libraries");
    let current_directory = env::current_dir().unwrap();

    _ = fs::create_dir(&libraries);

    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let suffix = env::consts::DLL_EXTENSION;

    let i = Arc::new(Mutex::new(0));

    let first = i.clone();
    let eva_dll = ["runa.", suffix].concat();
    let eva_temp_file = libraries.join(&eva_dll);
    let eva_temp_file_clone = eva_temp_file.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let runa = [CDN, "runa-", os, "-", arch, ".", suffix].concat();

                if let Err(err) = cdn::download(&runa, &eva_temp_file_clone).await {
                    println!("failed when download Runa: {err} as {runa}");
                    return;
                }

                *first.lock().unwrap() += 1;
            });
    });

    let second = i.clone();
    let eva_dll = ["eva.", suffix].concat();
    let eva_temp_file = libraries.join(&eva_dll);
    let eva_temp_file_clone = eva_temp_file.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let eva = [CDN, "eva-", os, "-", arch, ".", suffix].concat();

                if let Err(err) = cdn::download(&eva, &eva_temp_file_clone).await {
                    println!("failed when download Eva: {err} as {eva}");
                    return;
                }

                *second.lock().unwrap() += 1;
            });
    });

    let installation_path = state.std_path.clone().trim().to_string();
    let sink = ctx.get_external_handle();
    std::thread::spawn(move || {
        while *i.lock().unwrap() != 2 {};

        _ = sink.submit_command(SET_PROGRESS, 0.1, Target::Auto);

        const CARLA: &'static str = "https://github.com/lucasFelixSilveira/carla/archive/refs/heads/main.zip";
        let carla_temp_file = libraries.join("carla.zip");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                if let Err(err) = cdn::download(&CARLA, &carla_temp_file).await {
                    println!("failed when download Carla: {err} as {CARLA}");
                    return;
                }
                _ = sink.submit_command(SET_PROGRESS, 0.3, Target::Auto);
            });

        let carla_unpack = libraries.join("carla.unpack");
        let carla_zip = File::open(&carla_temp_file).unwrap();

        _ = fs::create_dir(&carla_unpack);

        let mut archive = ZipArchive::new(BufReader::new(carla_zip)).unwrap();

        for i in 0..archive.len() {
            let mut file_in_zip = archive.by_index(i).unwrap();
            let outpath = carla_unpack.join(file_in_zip.sanitized_name());

            if file_in_zip.is_dir() {
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                let mut outfile = BufWriter::new(File::create(&outpath).unwrap());
                std::io::copy(&mut file_in_zip, &mut outfile).unwrap();
            }
        }

        println!("Unpacked (.zip) from carla.");
        _ = sink.submit_command(SET_PROGRESS, 0.3, Target::Auto);

        let carla_main = carla_unpack.join("carla-main");
        let carla_compiler = carla_main.join("compiler");
        _ = std::env::set_current_dir(&carla_compiler);

        println!("Defined {} as the current directory", carla_compiler.display());

        #[cfg(target_os = "linux")]
        #[cfg(target_arch = "x86_64")]
        run(&format!(
            "g++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp -o ../build/carla -I. -L./libs/x86_64-linux -leva -Wl,-rpath,'$ORIGIN'"
        ));

        #[cfg(target_os = "windows")]
        #[cfg(target_arch = "x86_64")]
        run(&format!(
            "g++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp -o ../build/carla -I. -L./libs/x86_64-windows -leva -Wl,-rpath,'$ORIGIN'"
        ));

        println!("Carla compiled.");

        let carla_binary = carla_main.join(["build/carla", BINARY_SUFFIX].concat());
        let carla_binary_dest = libraries.join(["carla", BINARY_SUFFIX].concat());

        _ = fs::remove_file(&carla_temp_file);
        _ = fs::copy(&carla_binary, carla_binary_dest).unwrap();
        _ = fs::remove_dir_all(&carla_main);

        _ = sink.submit_command(SET_PROGRESS, 0.4, Target::Auto);

        const MORGANA: &'static str = "https://github.com/lucasFelixSilveira/morgana/archive/refs/heads/main.zip";
        let morgana_temp_file = libraries.join("morgana.zip");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                if let Err(err) = cdn::download(&MORGANA, &morgana_temp_file).await {
                    println!("failed when download Morgana: {err} as {CARLA}");
                    return;
                }
                _ = sink.submit_command(SET_PROGRESS, 0.6, Target::Auto);
            });

        let morgana_zip = File::open(&morgana_temp_file).unwrap();

        let mut archive = ZipArchive::new(BufReader::new(morgana_zip)).unwrap();

        for i in 0..archive.len() {
            let mut file_in_zip = archive.by_index(i).unwrap();
            let outpath = carla_unpack.join(file_in_zip.sanitized_name());

            if file_in_zip.is_dir() {
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                let mut outfile = BufWriter::new(File::create(&outpath).unwrap());
                std::io::copy(&mut file_in_zip, &mut outfile).unwrap();
            }
        }

        println!("Unpacked (.zip) from Morgana.");
        _ = sink.submit_command(SET_PROGRESS, 0.3, Target::Auto);

        let morgana_main = carla_unpack.join("morgana-main");
        let morgana_compiler = morgana_main.join("compiler");
        _ = std::env::set_current_dir(&morgana_compiler);

        println!("Defined {} as the current directory", morgana_compiler.display());

        #[cfg(target_os = "linux")]
        #[cfg(target_arch = "x86_64")]
        run(&format!(
            "g++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp -o ../bin/morgana -I. -L./libs/x86_64-linux -leva -lruna -Wl,-rpath,'$ORIGIN'"
        ));

        #[cfg(target_os = "windows")]
        #[cfg(target_arch = "x86_64")]
        run(&format!(
            "g++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp -o ../bin/morgana -I. -L./libs/x86_64-windows -leva -lruna -Wl,-rpath,'$ORIGIN'"
        ));

        println!("Morgana compiled.");

        let morgana_binary = morgana_main.join(["bin/morgana", BINARY_SUFFIX].concat());
        let morgana_binary_dest = libraries.join(["morgana", BINARY_SUFFIX].concat());

        _ = fs::remove_file(&morgana_temp_file);
        _ = fs::copy(&morgana_binary, morgana_binary_dest).unwrap();
        _ = fs::remove_dir_all(&morgana_main);

        _ = sink.submit_command(SET_PROGRESS, 0.85, Target::Auto);

        _ = fs::remove_dir(carla_unpack);

        if check::installation() {
            let installation = check::get_installation();
            _ = fs::remove_dir_all(installation);
        }

        println!("saving path `{installation_path}` as the installation path.");

        check::set_installation(&installation_path);

        let install_path = Path::new(&installation_path);
        _ = fs::create_dir(&install_path);

        let extensors = install_path.join("extensors");
        _ = fs::create_dir(&extensors);

        let bin_path = install_path.join("bin");
        _ = fs::create_dir(&bin_path);

        for entry in fs::read_dir(&libraries).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let dest_path =
                    if file_name.to_string_lossy().to_string().ends_with(DLL_SUFFIX) {
                        bin_path.join([DLL_PREFIX, file_name.to_str().unwrap()].concat())
                    } else { bin_path.join(file_name) };

                _ = fs::copy(&path, &dest_path);
                println!("Copied: {:?}", dest_path);
            }
        }

        _ = sink.submit_command(SET_PROGRESS, 0.95, Target::Auto);
        crate::utils::setup_path(bin_path.to_string_lossy().to_string());

        std::thread::sleep(std::time::Duration::from_secs(3));
        _ = sink.submit_command(SET_PROGRESS, 0.1, Target::Auto);
        std::thread::sleep(std::time::Duration::from_secs(5));

        Notification::new()
                .summary("Carla & Morgana Installer")
                .body("Instalation finished. The installer was closed.")
                .show()
                .unwrap();


        let (tx, rx) = mpsc::channel();
        sink.submit_command(
            GET_ALERT,
            tx,
            Target::Auto,
        ).unwrap();

        let active = rx.recv().unwrap();
        if active {
            println!("\"notify me\" was active");
            crate::utils::notification_sound();
        }

        std::process::exit(0);
    });


}
