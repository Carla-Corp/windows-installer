use druid::piet::TextStorage;
use druid::*;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use privesc::PrivilegedCommand;
use std::process::Command;

use crate::*;

static mut TIMES: usize = 0;

pub fn finish(sink: ExtEventSink, installation: &str) {
    if unsafe { TIMES } == 0 {
        unsafe { TIMES += 1 };
        return;
    }

    _ = sink.submit_command(SET_PROGRESS, 1f64, Target::Auto);

    let data = ArcStr::from(installation.clone());
    let arc_sink = Arc::from(sink);
    std::thread::spawn(move || {
        if let Err(e) = perform_file_operations(arc_sink, data) {
            eprintln!("fail in file operation: {}", e);
        }
    });
}

fn perform_file_operations(sink: Arc<ExtEventSink>, installation: ArcStr) -> io::Result<()> {
    let carla_path = PathBuf::from(installation.as_str());

    if carla_path.exists() {
        fs::remove_dir_all(&carla_path)?;
    }

    fs::create_dir_all(&carla_path)?;

    let bin_path = carla_path.join("bin");
    fs::create_dir_all(&bin_path)?;

    let carla_exe_source = PathBuf::from("temp/carla-main/build/carla.exe");
    let carla_exe_dest = bin_path.join("carla.exe");

    if carla_exe_source.exists() {
        fs::copy(&carla_exe_source, &carla_exe_dest)?;
    }

    let morgana_exe_source = PathBuf::from("temp/morgana-main/bin/morgana.exe");
    let morgana_exe_dest = bin_path.join("morgana.exe");

    if morgana_exe_source.exists() {
        fs::copy(&morgana_exe_source, &morgana_exe_dest)?;
    }

    let morgana_libs_source = PathBuf::from("temp/morgana-main/compiler/libs/x86_64-windows");
    let morgana_libs_runa = bin_path.join("runa.dll");
    let morgana_libs_eva = bin_path.join("eva.dll");
    let morgana_libs_runa_dest = bin_path.join("runa.dll");
    let morgana_libs_eva_dest = bin_path.join("eva.dll");
    if morgana_libs_source.exists() {
        let Ok(_) = fs::copy(&morgana_libs_runa, &morgana_libs_runa_dest) else {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to copy runa.dll: {}", morgana_libs_runa.display())));
        };

        let Ok(_) = fs::copy(&morgana_libs_eva, &morgana_libs_eva_dest) else {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to copy eva.dll: {}", morgana_libs_eva.display())));
        };
    }


    let extensors_path = carla_path.join("extensors");
    fs::create_dir_all(&extensors_path)?;

    let carla_zip = PathBuf::from("carla.zip");
    if carla_zip.exists() {
        fs::remove_file(&carla_zip)?;
    }

    let morgana_zip = PathBuf::from("morgana.zip");
    if morgana_zip.exists() {
        fs::remove_file(&morgana_zip)?;
    }

    let temp_path = PathBuf::from("temp");
    if temp_path.exists() {
        fs::remove_dir_all(&temp_path)?;
    }

    let bin_path_str = bin_path.to_str().unwrap();

    let script = format!(
        r#"
        $path = [Environment]::GetEnvironmentVariable("PATH", "Machine")
        if ($path -notlike "*{}*") {{
            [Environment]::SetEnvironmentVariable("PATH", "$path;{}", "Machine")
            Write-Host "OK"
        }}
        "#,
        bin_path_str, bin_path_str
    );

    let output =
        PrivilegedCommand::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
            .args(["-NoProfile", "-Command", &script])
            .gui(true)
            .run()
            .unwrap();

    _ = sink.submit_command(COMPLETED, false, Target::Auto);

    Ok(())
}
