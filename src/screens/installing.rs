use druid::*;
use widget::*;

use crate::{macros::*, screens::AppState};

pub fn page() -> impl Widget<AppState> {
    let installation = crate::check::installation();

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            std_padding!(
                Label::new("Carla & Morgana installer")
                    .with_text_size(KeyOrValue::Concrete(32.0))
            )
        )
        .with_child(
            std_padding_less_text!(
                Label::new(
                    if installation { "Working to reinstall (update) Carla and Morgana." }
                    else { "Working to install your first Carla and Morgana." }
                )
                    .with_text_size(KeyOrValue::Concrete(16.0))
                    .with_line_break_mode(LineBreaking::WordWrap)
            )
        )
        .with_child(
            ProgressBar::new()
                .lens(AppState::progress)
                .fix_width(500f64)
                .padding(KeyOrValue::Concrete(Insets {
                    x0: 30f64,
                    y0: 10f64,
                    x1: 30f64,
                    y1: 0f64,
                }))
                .align_left(),
        )
        .with_child(
            Flex::row()
                .with_child(Checkbox::new("").lens(AppState::alert))
                .with_child(
                    Label::new("Alert me when finished").padding(KeyOrValue::Concrete(Insets {
                        x0: 10f64,
                        y0: 0f64,
                        x1: 10f64,
                        y1: 0f64,
                    })),
                )
                .padding(KeyOrValue::Concrete(Insets {
                    x0: 30f64,
                    y0: 20f64,
                    x1: 30f64,
                    y1: 5f64,
                }))
                .align_left(),
        )
}
