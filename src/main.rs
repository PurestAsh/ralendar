use std::path::PathBuf;

use gpui_kit::component::{
    ActiveTheme, Theme, ThemeRegistry, TitleBar,
};
use gpui_kit::{
    App, AppContext, Context, IntoElement, Render, SharedString,
    Window, WindowBounds, WindowDecorations, WindowOptions, bounds,
    div, px, size,
};
use gpui_kit::{application, assets, init, open_window};

use crate::state::State;
use crate::test::test;
use crate::ui::ui;
mod icon;
mod side_bar;
mod state;
mod style;
mod test;
mod title_bar;
mod ui;

impl Render for State {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        ui(self, cx)
        // test()
    }
}

pub fn init_theme(cx: &mut App) {
    let theme_name = SharedString::from("Ayu Dark");
    // Load and watch themes from ./themes directory
    ThemeRegistry::watch_dir(PathBuf::from("theme"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned()
        {
            Theme::update(cx, |current| current.apply_config(&theme));
        }
    })
    .unwrap()
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);
        init_theme(cx);
        gpui_kit::component::set_locale("zh-CN");

        open_window(
            WindowOptions {
                window_decorations: Some(WindowDecorations::Client),
                window_bounds: Some(WindowBounds::centered(
                    size(px(1200.), px(700.)),
                    cx,
                )),
                ..TitleBar::window_options()
            },
            cx,
            |window, cx| cx.new(|cx| State::new(window, cx)),
        )
        .expect("Failed to open window");
    });
}
