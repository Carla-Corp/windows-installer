use druid::piet::TextStorage;
use druid::*;
use widget::*;

use std::process::Command;

use crate::notify::notify;
use crate::utils::beep;
use crate::*;

use crate::install::dependencies;

static mut INSTALLING: bool = true;

pub fn page() -> impl Widget<AppState> {
    let column = Flex::column();

    let title = "Carla & Morgana installer";
    let subtitle = "Enter the location where Carla and Morgana will be installed.";
    column
        .with_child(
            Label::new(title)
                .with_text_size(KeyOrValue::Concrete(32.0))
                .align_left()
                .padding(KeyOrValue::Concrete(Insets {
                    x0: 30f64,
                    y0: 20f64,
                    x1: 30f64,
                    y1: 20f64,
                })),
        )
        .with_child(
            Label::new(subtitle)
                .with_text_size(KeyOrValue::Concrete(16.0))
                .align_left()
                .padding(KeyOrValue::Concrete(Insets {
                    x0: 30f64,
                    y0: 5f64,
                    x1: 30f64,
                    y1: 5f64,
                })),
        )
        .with_child(
            TextBox::new()
                .with_placeholder("Enter installation path")
                .lens(AppState::install)
                .fix_width(500f64)
                .padding(KeyOrValue::Concrete(Insets {
                    x0: 30f64,
                    y0: 5f64,
                    x1: 30f64,
                    y1: 5f64,
                }))
                .align_left(),
        )
        .with_child(
            Button::new("Install")
                .on_click(|ctx, state: &mut AppState, _| {
                    if state.install.is_empty() {
                        beep();
                        return;
                    }

                    if !dependencies::check() {
                        show_error("Missing dependencies");
                        Command::new("cmd")
                            .arg("/c")
                            .arg("start")
                            .arg("msedge")
                            .arg("https:/github.com/Carla-corp/dependencies")
                            .spawn()
                            .ok();
                        return;
                    }

                    state.page = 1;

                    let carla_install = ArcStr::from(state.install.clone());
                    let carla_sink = ctx.get_external_handle();
                    _ = std::thread::spawn(move || {
                        println!("running carla");
                        install::carla::install(carla_sink, carla_install.as_str());
                    });

                    let morgana_install = ArcStr::from(state.install.clone());
                    let morgana_sink = ctx.get_external_handle();
                    _ = std::thread::spawn(move || {
                        println!("running morgana");
                        install::morgana::install(morgana_sink, morgana_install.as_str());
                    });
                })
                .padding(KeyOrValue::Concrete(Insets {
                    x0: 30f64,
                    y0: 5f64,
                    x1: 30f64,
                    y1: 5f64,
                }))
                .fix_width(300f64)
                .fix_height(40f64)
                .align_left(),
        )
}
