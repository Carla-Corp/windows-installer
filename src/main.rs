use druid::*;
use widget::*;

use crate::screens::AppState;
use std::sync::mpsc;

mod screens;
mod check;
mod macros;
mod install;
mod utils;

pub const SET_PROGRESS: Selector<f64> = Selector::new("carla.set-progress");
const GET_ALERT: Selector<mpsc::Sender<bool>> =
    Selector::new("carla.get-alert");

#[cfg(target_os = "windows")]
pub const INSTALLATION_PATH: &str = r"C:\.carla";

#[cfg(target_os = "linux")]
pub const INSTALLATION_PATH: &str = r"~/.carla";

#[cfg(target_os = "macos")]
pub const INSTALLATION_PATH: &str = r"/Users/Shared/.carla";

#[cfg(target_os = "freebsd")]
pub const INSTALLATION_PATH: &str = r"/usr/local/.carla";

pub struct AppController;

impl<W: Widget<AppState>> Controller<AppState, W> for AppController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
            if let Some(progress) = cmd.get(SET_PROGRESS) {
                data.progress = *progress;
                ctx.request_paint();
                return;
            }

            if let Some(sender) = cmd.get(GET_ALERT) {
                sender.send(data.alert).unwrap();
                return;
            }
        }

        child.event(ctx, event, data, env);
    }
}

fn main() {
    #[cfg(not(target_os = "windows"))]
    let window = WindowDesc::new(screens::ui_builder())
        .title("Carla & Morgana installer (GUI)")
        .window_size((700.0, 250.0))
        .resizable(false);

    #[cfg(target_os = "windows")]
    let window = WindowDesc::new(screens::ui_builder())
        .title("Carla & Morgana installer")
        .window_size((800.0, 250.0))
        .resizable(false);

    let state = screens::AppState {
        std_path: if check::installation() { check::get_installation() } else { INSTALLATION_PATH.to_string() },
        ..Default::default()
    };

    _ = AppLauncher::with_window(window)
        .launch(state);
}
