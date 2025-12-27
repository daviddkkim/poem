use gpui::*;

mod components;
mod views;

use components::text_editor::*;
use views::HelloWorld;

fn main() {
    Application::new().run(|cx: &mut App| {
        // Bind keys to actions for the text editor
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, Some("TextEditor")),
            KeyBinding::new("delete", Delete, Some("TextEditor")),
            KeyBinding::new("left", MoveLeft, Some("TextEditor")),
            KeyBinding::new("right", MoveRight, Some("TextEditor")),
            KeyBinding::new("home", MoveToStart, Some("TextEditor")),
            KeyBinding::new("end", MoveToEnd, Some("TextEditor")),
            KeyBinding::new("enter", Newline, Some("TextEditor")),
            KeyBinding::new("cmd-v", Paste, Some("TextEditor")),
            KeyBinding::new("cmd-c", Copy, Some("TextEditor")),
            KeyBinding::new("cmd-x", Cut, Some("TextEditor")),
            KeyBinding::new("cmd-s", Save, Some("TextEditor")),
            KeyBinding::new("cmd-z", Undo, Some("TextEditor")),
            KeyBinding::new("cmd-shift-z", Redo, Some("TextEditor")),
        ]);

        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| HelloWorld::new("World".into(), cx))
        })
        .unwrap();
    });
}
