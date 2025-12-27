use gpui::{prelude::*, *};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

#[derive(Clone, Debug)]
pub enum EntryKind {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub kind: EntryKind,
    pub children: Vec<Entry>,
    pub is_expanded: bool,
}

impl Entry {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let metadata = std::fs::metadata(path)?;

        if metadata.is_dir() {
            let mut children = Vec::new();

            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(child) = Self::from_path(entry.path()) {
                        children.push(child);
                    }
                }
            }

            // Sort: directories first, then files, alphabetically
            children.sort_by(|a, b| match (&a.kind, &b.kind) {
                (EntryKind::Directory, EntryKind::File) => std::cmp::Ordering::Less,
                (EntryKind::File, EntryKind::Directory) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });

            Ok(Self {
                path: path.to_path_buf(),
                name,
                kind: EntryKind::Directory,
                children,
                is_expanded: false,
            })
        } else {
            Ok(Self {
                path: path.to_path_buf(),
                name,
                kind: EntryKind::File,
                children: Vec::new(),
                is_expanded: false,
            })
        }
    }
}

use crate::components::TextEditor;

pub struct Worktree {
    focus_handle: FocusHandle,
    root: Entry,
    #[allow(dead_code)]
    root_path: PathBuf,
    #[allow(dead_code)]
    _watcher: Option<RecommendedWatcher>,
    #[allow(dead_code)]
    _receiver: Option<Receiver<notify::Result<Event>>>,
    editor: Option<Entity<TextEditor>>,
}

impl Worktree {
    pub fn new(path: impl AsRef<Path>, cx: &mut Context<Self>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let mut root = Entry::from_path(path)?;

        // Expand only the root directory by default
        root.is_expanded = true;

        // Set up filesystem watcher
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).ok();

        if let Some(ref mut w) = watcher {
            let _ = w.watch(path, RecursiveMode::Recursive);
        }

        Ok(Self {
            focus_handle: cx.focus_handle(),
            root,
            root_path: path.to_path_buf(),
            _watcher: watcher,
            _receiver: Some(rx),
            editor: None,
        })
    }

    pub fn set_editor(&mut self, editor: Entity<TextEditor>) {
        self.editor = Some(editor);
    }

    #[allow(dead_code)]
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        if let Ok(new_root) = Entry::from_path(&self.root_path) {
            // Preserve expansion state
            self.preserve_expansion_state(&mut self.root.clone(), &new_root);
            self.root = new_root;
            cx.notify();
        }
    }

    #[allow(dead_code)]
    fn preserve_expansion_state(&self, old_entry: &mut Entry, new_entry: &Entry) {
        if old_entry.path == new_entry.path && old_entry.is_expanded {
            // This path was expanded, keep it expanded
            for new_child in &new_entry.children {
                if let Some(old_child) = old_entry
                    .children
                    .iter_mut()
                    .find(|c| c.path == new_child.path)
                {
                    self.preserve_expansion_state(old_child, new_child);
                }
            }
        }
    }

    fn toggle_entry(&mut self, path: &PathBuf, cx: &mut Context<Self>) {
        Self::toggle_entry_recursive(&mut self.root, path);
        cx.notify();
    }

    fn toggle_entry_recursive(entry: &mut Entry, path: &PathBuf) -> bool {
        if &entry.path == path {
            entry.is_expanded = !entry.is_expanded;
            return true;
        }

        for child in &mut entry.children {
            if Self::toggle_entry_recursive(child, path) {
                return true;
            }
        }

        false
    }

    fn render_entry(&self, entry: &Entry, depth: usize, cx: &mut Context<Self>) -> Div {
        let path = entry.path.clone();
        let indent = depth * 20;
        let is_dir = matches!(entry.kind, EntryKind::Directory);
        let is_expanded = entry.is_expanded;

        let icon = if is_dir {
            if is_expanded {
                "▼"
            } else {
                "▶"
            }
        } else {
            "📄"
        };

        let mut container = div().flex().flex_col().child(
            div()
                .flex()
                .items_center()
                .px_2()
                .py_1()
                .pl(px(indent as f32))
                .hover(|style| style.bg(rgb(0xf0f0f0)))
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                        if is_dir {
                            this.toggle_entry(&path, cx);
                        } else if let Some(editor) = &this.editor {
                            let path_clone = path.clone();
                            editor.update(cx, |ed, cx| {
                                ed.open_file(path_clone, cx);
                            });
                        }
                    }),
                )
                .child(div().w(px(24.)).text_sm().child(icon))
                .child(
                    div()
                        .text_sm()
                        .text_color(if is_dir { rgb(0x0066cc) } else { rgb(0x333333) })
                        .when(is_dir, |div| div.font_weight(FontWeight::BOLD))
                        .child(entry.name.clone()),
                ),
        );

        // Render children if directory is expanded
        if is_dir && is_expanded {
            for child in &entry.children {
                container = container.child(self.render_entry(child, depth + 1, cx));
            }
        }

        container
    }
}

impl Focusable for Worktree {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Worktree {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(white())
            .overflow_hidden()
            .h_full()
            .track_focus(&self.focus_handle)
            .child(self.render_entry(&self.root.clone(), 0, cx))
    }
}
