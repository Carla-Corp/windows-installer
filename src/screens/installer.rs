use druid::*;
use widget::*;

use crate::AppState;
use crate::utils::*;

pub fn page() -> impl Widget<AppState> {
    let column = Flex::column();

    let title = "Carla & Morgana installer";
    let subtitle = "Please wait while the installation is being performed.";
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
            ProgressBar::new()
                .controller(ProgressController)
                .lens(AppState::progress)
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
