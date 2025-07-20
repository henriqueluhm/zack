//! Position type used throughout the Zack text editor.
//!
//! This module defines the [`Position`] struct, which represents a cursor or character location
//! in a text buffer using a zero-based `(line, column)` format. It's useful for cursor tracking,
//! highlighting, editing operations, and buffer navigation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Zero-based line index.
    pub line: usize,
    /// Zero-based column index.
    pub col: usize,
}

impl Position {
    /// Creates a new `Position` with the specified line and column.
    ///
    /// # Arguments
    ///
    /// * `line` - The line index (starting from 0).
    /// * `col` - The column index (starting from 0).
    ///
    /// # Returns
    ///
    /// A new [`Position`] instance.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
