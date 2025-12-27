use gpui::*;

mod components;
mod views;

use views::HelloWorld;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| HelloWorld::new("World".into(), cx))
        })
        .unwrap();
    });
}
