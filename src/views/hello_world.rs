use crate::components::{TextEditor, TextInput, Worktree};
use gpui::*;

pub struct HelloWorld {
    text: SharedString,
    text_input: Entity<TextInput>,
    styled_input: Entity<TextInput>,
    text_editor: Entity<TextEditor>,
    worktree: Option<Entity<Worktree>>,
}

impl HelloWorld {
    pub fn new(text: SharedString, cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| TextInput::new(cx));
        let styled_input = cx.new(|cx| TextInput::new(cx).placeholder("Enter your name..."));

        // Load the current project directory as worktree
        let worktree = cx.new(|cx| {
            Worktree::new(".", cx).unwrap_or_else(|_| {
                Worktree::new("/Users/davidkim/Apps/poem", cx).expect("Failed to load worktree")
            })
        });

        let text_editor = cx.new(|cx| {
            TextEditor::with_text("// Start typing here...\n// Arrow keys to move cursor\n// Backspace/Delete to remove text", cx)
        });

        Self {
            text,
            text_input,
            styled_input,
            text_editor,
            worktree: Some(worktree),
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0xffffff))
            .size_full()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child("Basic Text Editor (Rope-based):"),
            )
            .child(self.text_editor.clone())
    }
}

// .child(
//     div()
//         .flex()
//         .flex_col()
//         .gap_2()
//         .w(px(400.))
//         .child(
//             div()
//                 .text_sm()
//                 .text_color(rgb(0x666666))
//                 .child("Styled input (wrap in styled div):"),
//         )
//         .child(
//             // Wrap the input in a styled container
//             div()
//                 .w_full()
//                 .p_4()
//                 .bg(rgb(0xf0f9ff))
//                 .border_2()
//                 .border_color(rgb(0x3b82f6))
//                 .rounded_lg()
//                 .hover(|style| style.bg(rgb(0xe0f2fe)).shadow_md())
//                 .child(self.styled_input.clone()),
//         ),
