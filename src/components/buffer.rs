use gpui::*;
use ropey::Rope;
use std::path::PathBuf;

#[derive(Clone, Debug)]
enum Edit {
    Insert { pos: usize, text: String },
    Remove { pos: usize, text: String },
}

/// A Buffer represents a file's content and state
pub struct Buffer {
    /// The text content
    rope: Rope,
    /// Path to the file on disk (None for new/unsaved buffers)
    file_path: Option<PathBuf>,
    /// Whether the buffer has unsaved changes
    is_dirty: bool,
    /// Undo history
    undo_stack: Vec<Edit>,
    /// Redo history
    redo_stack: Vec<Edit>,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            is_dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Create a buffer with initial text
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            rope: Rope::from_str(&text.into()),
            file_path: None,
            is_dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Load a buffer from a file
    pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        Ok(Self {
            rope: Rope::from_str(&content),
            file_path: Some(path),
            is_dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
    }

    /// Save the buffer to its file
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.file_path {
            std::fs::write(path, self.rope.to_string())?;
            self.is_dirty = false;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path set",
            ))
        }
    }

    /// Save the buffer to a specific path
    pub fn save_as(&mut self, path: PathBuf) -> std::io::Result<()> {
        std::fs::write(&path, self.rope.to_string())?;
        self.file_path = Some(path);
        self.is_dirty = false;
        Ok(())
    }

    /// Get the buffer content as a string
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get the file path
    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    /// Get the file name
    pub fn file_name(&self) -> Option<&str> {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
    }

    /// Check if the buffer has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    /// Insert text at a position
    pub fn insert(&mut self, pos: usize, text: &str) {
        self.rope.insert(pos, text);
        self.undo_stack.push(Edit::Insert {
            pos,
            text: text.to_string(),
        });
        self.redo_stack.clear(); // Clear redo stack on new edit
        self.is_dirty = true;
    }

    /// Insert a character at a position
    pub fn insert_char(&mut self, pos: usize, c: char) {
        self.rope.insert_char(pos, c);
        self.undo_stack.push(Edit::Insert {
            pos,
            text: c.to_string(),
        });
        self.redo_stack.clear(); // Clear redo stack on new edit
        self.is_dirty = true;
    }

    /// Remove a range of text
    pub fn remove(&mut self, range: std::ops::Range<usize>) {
        let removed_text = self.rope.slice(range.start..range.end).to_string();
        self.rope.remove(range.clone());
        self.undo_stack.push(Edit::Remove {
            pos: range.start,
            text: removed_text,
        });
        self.redo_stack.clear(); // Clear redo stack on new edit
        self.is_dirty = true;
    }

    /// Get the length of the buffer in bytes
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Get the length of the buffer in chars
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get a reference to the underlying rope
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Replace the entire buffer content
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.rope = Rope::from_str(&text.into());
        self.is_dirty = true;
    }

    /// Load content from a file, replacing current content
    pub fn load_file(&mut self, path: PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(&path)?;
        self.rope = Rope::from_str(&content);
        self.file_path = Some(path);
        self.is_dirty = false;
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(())
    }

    /// Undo the last edit
    pub fn undo(&mut self) -> Option<usize> {
        let edit = self.undo_stack.pop()?;

        let cursor_pos = match &edit {
            Edit::Insert { pos, text } => {
                // Undo an insert by removing the text
                let end = pos + text.len();
                self.rope.remove(*pos..end);
                *pos
            }
            Edit::Remove { pos, text } => {
                // Undo a remove by inserting the text back
                self.rope.insert(*pos, text);
                *pos
            }
        };

        self.redo_stack.push(edit);
        Some(cursor_pos)
    }

    /// Redo the last undone edit
    pub fn redo(&mut self) -> Option<usize> {
        let edit = self.redo_stack.pop()?;

        let cursor_pos = match &edit {
            Edit::Insert { pos, text } => {
                // Redo an insert
                self.rope.insert(*pos, text);
                pos + text.len()
            }
            Edit::Remove { pos, text } => {
                // Redo a remove
                let end = pos + text.len();
                self.rope.remove(*pos..end);
                *pos
            }
        };

        self.undo_stack.push(edit);
        Some(cursor_pos)
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
