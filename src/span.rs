// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::{ops::Range, sync::Arc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug)]
pub struct Source {
    name: Option<String>,
    contents: String,
    lines: Vec<usize>,
}

impl Source {
    #[must_use]
    pub fn new(name: Option<String>, contents: String) -> Self {
        let lines = contents
            .match_indices('\n')
            .map(|(i, _)| i)
            .chain(std::iter::once(contents.len()))
            .collect();

        Self {
            name,
            contents,
            lines,
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    #[must_use]
    pub fn location(&self, byte: usize) -> Location {
        let line = self.lines.partition_point(|&x| x < byte);
        let start = line.checked_sub(1).map_or(0, |n| self.lines[n] + 1);
        let col = self.contents[start..byte].chars().count();

        Location::new(line, col)
    }

    #[must_use]
    pub fn contents(&self) -> &str {
        &self.contents
    }

    #[must_use]
    pub fn get_line(&self, line: usize) -> &str {
        let end = self.lines[line];
        let start = line.checked_sub(1).map_or(0, |n| self.lines[n] + 1);
        &self.contents[start..end]
    }
}

impl Default for Source {
    fn default() -> Self {
        Self::new(None, String::new())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Span {
    pub bytes: Range<usize>,
    pub source: Arc<Source>,
}

impl Span {
    #[must_use]
    pub fn new(bytes: Range<usize>, source: Arc<Source>) -> Self {
        Self { bytes, source }
    }

    #[must_use]
    pub fn join(self, other: &Self) -> Self {
        debug_assert!(self.same_source(other));

        Self::new(self.bytes.start..other.bytes.end, self.source)
    }

    pub fn extend(&mut self, other: &Self) {
        debug_assert!(self.same_source(other));

        self.bytes.end = other.bytes.end;
    }

    #[must_use]
    pub fn location(&self) -> Location {
        self.source.location(self.bytes.start)
    }

    #[must_use]
    pub fn end_location(&self) -> Location {
        self.source.location(self.bytes.end)
    }

    #[must_use]
    pub fn same_source(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.source, &other.source)
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.same_source(other) && self.bytes == other.bytes
    }
}
