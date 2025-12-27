use gpui::{prelude::*, *};

pub struct TextInput {
    focus_handle: FocusHandle,
    content: String,
    placeholder: String,
}

impl TextInput {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: String::new(),
            placeholder: "Type here...".to_string(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    #[allow(dead_code)]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[allow(dead_code)]
    pub fn set_content(&mut self, content: impl Into<String>, cx: &mut Context<Self>) {
        self.content = content.into();
        cx.notify();
    }

    fn handle_input(&mut self, input: &str, cx: &mut Context<Self>) {
        self.content.push_str(input);
        cx.notify();
    }

    fn handle_backspace(&mut self, cx: &mut Context<Self>) {
        self.content.pop();
        cx.notify();
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = self.content.clone();
        let placeholder = self.placeholder.clone();
        let is_focused = self.focus_handle.is_focused(_window);

        div()
            .key_context("TextInput")
            .track_focus(&self.focus_handle)
            .cursor(CursorStyle::IBeam)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if event.keystroke.key == "backspace" {
                    this.handle_backspace(cx);
                } else if event.keystroke.key == "space" {
                    this.handle_input(" ", cx);
                } else if event.keystroke.key.len() == 1 {
                    this.handle_input(&event.keystroke.key, cx);
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event: &MouseDownEvent, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }),
            )
            // Default minimal styling - can be overridden at call site
            .px_3()
            .py_2()
            .bg(white())
            .border_1()
            .border_color(rgb(0xcccccc))
            .rounded_md()
            .when(is_focused, |div: Div| div.border_color(rgb(0x0066ff)))
            .child(if content.is_empty() {
                div().text_color(rgb(0x999999)).child(placeholder)
            } else {
                div().child(content)
            })
    }
}
