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

        let text_editor = cx.new(|cx| {
            TextEditor::with_text(
                "// Click a file in the worktree to open it\n// Cmd+S to save",
                cx,
            )
        });

        // Load the current project directory as worktree
        let worktree = cx.new(|cx| {
            Worktree::new(".", cx).unwrap_or_else(|_| {
                Worktree::new("/Users/davidkim/Apps/poem", cx).expect("Failed to load worktree")
            })
        });

        // Set the editor reference in the worktree so it can open files
        worktree.update(cx, |tree, _cx| {
            tree.set_editor(text_editor.clone());
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
        let mut root = div().flex().flex_row().bg(rgb(0xffffff)).size_full();

        // Add worktree on the left if it exists
        if let Some(worktree) = &self.worktree {
            root = root.child(
                div()
                    .w(px(300.))
                    .h_full()
                    .border_r_1()
                    .border_color(rgb(0xe5e5e5))
                    .overflow_hidden()
                    .child(worktree.clone()),
            );
        }

        // Add text editor on the right
        root.child(
            div()
                .flex_1()
                .h_full()
                .p_4()
                .child(self.text_editor.clone()),
        )
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
