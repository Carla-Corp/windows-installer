use druid::*;
use widget::*;

use crate::*;

pub fn page() -> impl Widget<AppState> {
    let column = Flex::column();

    let title = "Carla & Morgana installer";
    let subtitle = "Installation complete";
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
}
