use crate::components::Buffer;
use gpui::{prelude::*, *};
use std::path::PathBuf;

// Define actions for the text editor
actions!(
    text_editor,
    [
        Backspace,
        Delete,
        MoveLeft,
        MoveRight,
        MoveToStart,
        MoveToEnd,
        Newline,
        Paste,
        Copy,
        Cut,
        Save,
        Undo,
        Redo,
    ]
);

/// A text editor component that provides UI for editing a Buffer
pub struct TextEditor {
    focus_handle: FocusHandle,
    buffer: Buffer,
    cursor: usize, // Cursor position in bytes
}

impl TextEditor {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            buffer: Buffer::new(),
            cursor: 0,
        }
    }

    pub fn with_text(text: impl Into<String>, cx: &mut Context<Self>) -> Self {
        let buffer = Buffer::with_text(text);
        let cursor = buffer.len_bytes();
        Self {
            focus_handle: cx.focus_handle(),
            buffer,
            cursor,
        }
    }

    pub fn open_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if let Ok(()) = self.buffer.load_file(path) {
            self.cursor = 0;
            cx.notify();
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    fn insert_char(&mut self, c: char, cx: &mut Context<Self>) {
        self.buffer.insert_char(self.cursor, c);
        self.cursor += c.len_utf8();
        cx.notify();
    }

    fn insert_text(&mut self, text: &str, cx: &mut Context<Self>) {
        self.buffer.insert(self.cursor, text);
        self.cursor += text.len();
        cx.notify();
    }

    // Action handlers
    fn backspace(&mut self, _: &Backspace, _window: &mut Window, cx: &mut Context<Self>) {
        if self.cursor > 0 {
            let text = self.buffer.to_string();
            if let Some((idx, _)) = text[..self.cursor].char_indices().next_back() {
                self.buffer.remove(idx..self.cursor);
                self.cursor = idx;
                cx.notify();
            }
        }
    }

    fn delete(&mut self, _: &Delete, _window: &mut Window, cx: &mut Context<Self>) {
        if self.cursor < self.buffer.len_chars() {
            let text = self.buffer.to_string();
            if let Some((_, c)) = text[self.cursor..].char_indices().next() {
                self.buffer.remove(self.cursor..self.cursor + c.len_utf8());
                cx.notify();
            }
        }
    }

    fn move_left(&mut self, _: &MoveLeft, _window: &mut Window, cx: &mut Context<Self>) {
        if self.cursor > 0 {
            let text = self.buffer.to_string();
            if let Some((idx, _)) = text[..self.cursor].char_indices().next_back() {
                self.cursor = idx;
                cx.notify();
            }
        }
    }

    fn move_right(&mut self, _: &MoveRight, _window: &mut Window, cx: &mut Context<Self>) {
        let text = self.buffer.to_string();
        if self.cursor < text.len() {
            if let Some((_, c)) = text[self.cursor..].char_indices().next() {
                self.cursor += c.len_utf8();
                cx.notify();
            }
        }
    }

    fn move_to_start(&mut self, _: &MoveToStart, _window: &mut Window, cx: &mut Context<Self>) {
        self.cursor = 0;
        cx.notify();
    }

    fn move_to_end(&mut self, _: &MoveToEnd, _window: &mut Window, cx: &mut Context<Self>) {
        self.cursor = self.buffer.to_string().len();
        cx.notify();
    }

    fn newline(&mut self, _: &Newline, _window: &mut Window, cx: &mut Context<Self>) {
        self.insert_char('\n', cx);
    }

    fn paste(&mut self, _: &Paste, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.insert_text(&text, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _window: &mut Window, cx: &mut Context<Self>) {
        let text = self.buffer.to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn cut(&mut self, _: &Cut, _window: &mut Window, cx: &mut Context<Self>) {
        let text = self.buffer.to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(text));
        self.buffer.set_text("");
        self.cursor = 0;
        cx.notify();
    }

    fn save(&mut self, _: &Save, _window: &mut Window, cx: &mut Context<Self>) {
        if self.buffer.save().is_ok() {
            cx.notify();
        }
    }

    fn undo(&mut self, _: &Undo, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_cursor) = self.buffer.undo() {
            self.cursor = new_cursor;
            cx.notify();
        }
    }

    fn redo(&mut self, _: &Redo, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_cursor) = self.buffer.redo() {
            self.cursor = new_cursor;
            cx.notify();
        }
    }

    fn render_text_with_cursor(&self, is_focused: bool) -> String {
        let text = self.buffer.to_string();

        if !is_focused {
            // Don't show cursor when not focused
            return if text.is_empty() { String::new() } else { text };
        }

        if text.is_empty() {
            return "▎".to_string();
        }

        // Insert cursor marker at cursor position
        let before_cursor = &text[..self.cursor];
        let after_cursor = &text[self.cursor..];

        format!("{}▎{}", before_cursor, after_cursor)
    }
}

impl Focusable for TextEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(_window);
        let content = self.render_text_with_cursor(is_focused);

        let file_name = self.buffer.file_name().unwrap_or("Untitled");

        let dirty_indicator = if self.buffer.is_dirty() { " ●" } else { "" };

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            // Header with file name
            .child(
                div()
                    .flex()
                    .items_center()
                    .p_2()
                    .border_b_1()
                    .border_color(rgb(0xe5e5e5))
                    .bg(rgb(0xf9f9f9))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .child(format!("{}{}", file_name, dirty_indicator)),
                    ),
            )
            // Editor content
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .key_context("TextEditor")
                    .track_focus(&self.focus_handle)
                    // Register action handlers
                    .on_action(cx.listener(Self::backspace))
                    .on_action(cx.listener(Self::delete))
                    .on_action(cx.listener(Self::move_left))
                    .on_action(cx.listener(Self::move_right))
                    .on_action(cx.listener(Self::move_to_start))
                    .on_action(cx.listener(Self::move_to_end))
                    .on_action(cx.listener(Self::newline))
                    .on_action(cx.listener(Self::paste))
                    .on_action(cx.listener(Self::copy))
                    .on_action(cx.listener(Self::cut))
                    .on_action(cx.listener(Self::save))
                    .on_action(cx.listener(Self::undo))
                    .on_action(cx.listener(Self::redo))
                    // Handle regular character input
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                        // Only handle regular character input here
                        // Actions handle special keys
                        let key = event.keystroke.key.as_str();
                        if key == "space" {
                            this.insert_char(' ', cx);
                        } else if key.len() == 1 {
                            if let Some(c) = key.chars().next() {
                                // Only insert if it's a printable character
                                if !c.is_control() {
                                    this.insert_char(c, cx);
                                }
                            }
                        }
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event: &MouseDownEvent, window, cx| {
                            window.focus(&this.focus_handle, cx);
                        }),
                    )
                    .cursor(CursorStyle::IBeam)
                    // Styling
                    .p_4()
                    .bg(white())
                    // .border_1()
                    // .border_color(rgb(0xcccccc))
                    .rounded_md()
                    .w_full()
                    .h_full()
                    .font_family("monospace")
                    .text_base()
                    // .when(is_focused, |div: Div| {
                    //     div.border_color(rgb(0x0066ff)).border_2()
                    // })
                    .child(content),
            )
    }
}
