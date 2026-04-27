use druid::widget::*;
use druid::*;

mod install;
mod notify;
mod screens;
mod utils;

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    install: String,
    page: usize,
    progress: f64,
    alert: bool,
    a: i8,
}

pub const SET_PROGRESS: Selector<f64> = Selector::new("carla.set-progress");
pub const SET_PAGE: Selector<usize> = Selector::new("carla.set-page");
pub const SET_ALERT: Selector<bool> = Selector::new("carla.set-alert");
pub const COMPLETED: Selector<bool> = Selector::new("carla.completed");

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

            if let Some(page) = cmd.get(SET_PAGE) {
                data.page = *page;
                ctx.request_layout();
                return;
            }

            if let Some(alert) = cmd.get(SET_ALERT) {
                data.alert = *alert;
                return;
            }

            if let Some(_) = cmd.get(COMPLETED) {
                if data.alert {
                    notify::notify("Carla & Morgana sent a message", "finished installation");
                }
                data.page = 2;
                return;
            }
        }

        child.event(ctx, event, data, env);
    }
}

fn main() {
    let state = AppState {
        install: r"C:\.carla".into(),
        progress: 0.03,
        ..Default::default()
    };

    let main_window = WindowDesc::new(ui_builder())
        .resizable(false)
        .title("Carla & Morgana Installer")
        .window_size((600.0, 250.0));

    AppLauncher::with_window(main_window).launch(state).unwrap();
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(
            Either::new(
                |data: &AppState, _| data.page == 0,
                screens::home::page(),
                Either::new(
                    |data: &AppState, _| data.page == 1,
                    screens::installer::page(),
                    screens::finished::page(),
                ),
            ),
            1.0,
        )
        .controller(AppController)
}

pub fn show_error(err: impl ToString) {
    notify::notify("Carla & Morgana Installer sent an error", &err.to_string());
}
