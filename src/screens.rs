use druid::*;
use widget::*;

mod home;
mod installing;

#[derive(Default, Clone, Debug, PartialEq, Eq, Data)]
pub enum Page {
    Installing,

    #[default]
    Home,
}

#[derive(Data, Default, Clone, Debug, Lens)]
pub struct AppState {
    pub page: Page,
    pub std_path: String,
    pub progress: f64,
    pub alert: bool,
}

pub fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(
            ViewSwitcher::new(
                |data: &AppState, _| data.page.clone(),
                |selector, _, _| match selector {
                    Page::Home => Box::new(home::page()),
                    Page::Installing => Box::new(installing::page()),
                },
            ),
            1.0,
        ).controller(crate::AppController)
}
