use druid::*;
use rfd::FileDialog;
use widget::*;

use crate::{install, macros::*, screens::{AppState, Page}};

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
                    if installation { "After a verification, was possible to identify you have an Carla and Morgana installation." }
                    else { "After a verification, was possible to identify you don't have an Carla and Morgana installation." }
                )
                    .with_text_size(KeyOrValue::Concrete(16.0))
                    .with_line_break_mode(LineBreaking::WordWrap)
            )
        )
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row()
                        .with_spacer(KeyOrValue::Concrete(2.0))
                        .with_child(
                            Container::new(
                                Flex::row()
                                    .with_spacer(KeyOrValue::Concrete(2.0))
                                    .with_child(
                                        std_padding_less!(
                                            TextBox::new()
                                                .with_placeholder("Enter installation path")
                                                .lens(AppState::std_path)
                                                .fix_width(500f64)
                                                .align_left()
                                        )
                                    )
                            )
                        )
                        .with_child(
                            Button::new("Find folder")
                                .on_click(move |_, state: &mut AppState, _| {
                                    let folder = FileDialog::new()
                                            .set_directory("/")
                                            .pick_folder();

                                    let Some(path) = folder else { return; };

                                    if !installation {
                                        state.std_path =
                                            path.join(".carla")
                                                .to_string_lossy()
                                                .to_string();
                                    }
                                })
                                .fix_height(30.0)
                        )
                        .padding(KeyOrValue::Concrete(Insets { x0: 0.0, y0: 10.0, x1: 0.0, y1: 5.0 }))
                )
            .with_child(
                std_padding_less!(
                    Button::new(if installation { "Reinstall (update)" } else { "Install" })
                        .on_click(move |ctx, state: &mut AppState, _| {
                            if installation && state.std_path != crate::check::get_installation() {
                                rfd::MessageDialog::new()
                                    .set_title("Reinstall")
                                    .set_description("You already have an installation and before pressing the reinstall button, the path was changed. The previous installation path will be used.")
                                    .show();

                                state.std_path = crate::check::get_installation();
                            }

                            state.page = Page::Installing;
                            install::install(ctx, state);
                        })
                        .fix_width(300f64)
                        .fix_height(30f64)
                        .align_left()
                )
            )
        )
}
