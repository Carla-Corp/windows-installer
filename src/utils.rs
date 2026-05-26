use std::env;
use std::fs;
use std::path::Path;

#[cfg(not(target_os = "windows"))]
pub fn setup_path(dir: String) {
    let home = env::var("HOME").expect("HOME not set");
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    let rc_file = if shell.contains("zsh") {
        let zdotdir = env::var("ZDOTDIR").ok();
        let base = zdotdir.as_deref().unwrap_or(&home);
        format!("{}/.zshrc", base)
    } else if shell.contains("bash") {
        format!("{}/.bashrc", home)
    } else if shell.contains("fish") {
        format!("{}/.config/fish/config.fish", home)
    } else {
        format!("{}/.profile", home)
    };

    if let Some(parent) = Path::new(&rc_file).parent() {
        fs::create_dir_all(parent).ok();
    }

    let mut content = fs::read_to_string(&rc_file).unwrap_or_default();

    if !Path::new(&dir).exists() { return; }

    let pattern = format!("export PATH=.*{}.*", dir.replace("/", "\\/"));
    let re = regex::Regex::new(&pattern).unwrap();
    content = re.replace_all(&content, "").to_string();

    let export = if shell.contains("fish") {
        format!("set -gx PATH {} $PATH\n", dir)
    } else {
        format!("export PATH=\"{}:$PATH\"\n", dir)
    };

    content.push_str(&format!("\n# Added by Carla installer\n{}\n", export));

    fs::write(&rc_file, content).unwrap();

    println!("✓ PATH configured for shell: {}", shell);
    println!("✓ RC file: {}", rc_file);
    println!("\nRun: source {}", rc_file);
}

#[cfg(target_os = "windows")]
pub fn setup_path(dir: String) {
    use privesc::PrivilegedCommand;

    let script = format!(
        r#"
        $path = [Environment]::GetEnvironmentVariable("PATH", "Machine")
        if ($path -notlike "*{}*") {{
            [Environment]::SetEnvironmentVariable("PATH", "$path;{}", "Machine")
            Write-Host "OK"
        }}
        "#,
        dir, dir
    );

    _ = PrivilegedCommand::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
            .args(["-NoProfile", "-Command", &script])
            .gui(true)
            .run()
            .unwrap();
}


#[cfg(windows)]
pub fn notification_sound() {
    use windows::Win32::UI::WindowsAndMessaging::MessageBeep;

    unsafe {
        MessageBeep(0xFFFFFFFF);
    }
}

#[cfg(target_os = "linux")]
pub fn notification_sound() {
    std::process::Command::new("canberra-gtk-play")
        .args(["-i", "message"])
        .spawn()
        .unwrap();
}

#[cfg(target_os = "macos")]
pub fn notification_sound() {
    std::process::Command::new("afplay")
        .arg("/System/Library/Sounds/Glass.aiff")
        .spawn()
        .unwrap();
}
