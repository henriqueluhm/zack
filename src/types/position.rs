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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_position_with_correct_values() {
        let pos = Position::new(3, 7);
        assert_eq!(pos.line, 3);
        assert_eq!(pos.col, 7);
    }

    #[test]
    fn should_be_equal_when_same_line_and_col() {
        let a = Position::new(0, 0);
        let b = Position::new(0, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn should_not_be_equal_when_different() {
        let a = Position::new(1, 2);
        let b = Position::new(2, 1);
        assert_ne!(a, b);
    }

    #[test]
    fn should_be_copy_and_clone() {
        let a = Position::new(4, 4);
        let b = a;
        let c = a.clone();
        assert_eq!(a, b);
        assert_eq!(a, c);
    }
}
