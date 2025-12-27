use crate::components::{TextInput, Worktree};
use gpui::*;

pub struct HelloWorld {
    text: SharedString,
    text_input: Entity<TextInput>,
    styled_input: Entity<TextInput>,
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

        Self {
            text,
            text_input,
            styled_input,
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
                // Left sidebar - Worktree
                div()
                    .flex()
                    .flex_col()
                    .w(px(300.))
                    .h_full()
                    .border_r_1()
                    .border_color(rgb(0xcccccc))
                    .bg(rgb(0xfafafa))
                    .p_2()
                    .child(if let Some(worktree) = self.worktree.clone() {
                        div().child(worktree)
                    } else {
                        div().child("No worktree loaded")
                    }),
            )
            .child(
                // Right content area
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .justify_center()
                    .items_center()
                    .gap_6()
                    .p_8()
                    .child(
                        div().bg(rgb(0x000000)).p_4().rounded_lg().child(
                            div()
                                .bg(rgb(0xd5d5d5))
                                .p_8()
                                .rounded_md()
                                .child(format!("Hello, {}!", &self.text)),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .w(px(400.))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child("Basic input (default styling):"),
                            )
                            .child(self.text_input.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .w(px(400.))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child("Styled input (wrap in styled div):"),
                            )
                            .child(
                                // Wrap the input in a styled container
                                div()
                                    .w_full()
                                    .p_4()
                                    .bg(rgb(0xf0f9ff))
                                    .border_2()
                                    .border_color(rgb(0x3b82f6))
                                    .rounded_lg()
                                    .hover(|style| style.bg(rgb(0xe0f2fe)).shadow_md())
                                    .child(self.styled_input.clone()),
                            ),
                    )
                    .child(
                        div()
                            .p_4()
                            .text_base()
                            .text_color(rgb(0x374151))
                            .child("Click folders in the sidebar to expand/collapse!"),
                    ),
            )
    }
}
