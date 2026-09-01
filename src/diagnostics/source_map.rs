use crate::diagnostics::Span;

pub struct SourceMap {
    pub source: String,
    ///Byte offset of each line start
    pub line_starts: Vec<usize>,
}

impl SourceMap {
    pub fn new(source: String) -> SourceMap {
        let line_starts = Self::get_line_starts(source.as_str());
        SourceMap {
            source,
            line_starts,
        }
    }

    ///This is a helper to precompute all the line starts the very first line is always at an index of 0
    fn get_line_starts(source: &str) -> Vec<usize> {
        let mut starts = vec![0];
        for (byte_pos, ch) in source.char_indices() {
            if ch == '\n' {
                starts.push(byte_pos + 1);
            }
        }
        starts
    }

    ///This is a helper that is meant to meant to get the exact line and col where a byte position is situated
    pub fn get_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let line = match self.line_starts.binary_search(&byte_pos) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };

        let col = byte_pos - self.line_starts[line];
        (line + 1, col + 1)
    }

    ///This gets the exact code snippet represented by the span from the source code
    pub fn get_snippet(&self, span: Span) -> String {
        //The mins are defensive clamping incase whoever is reporting is sending corrupted spans
        let start = span.start.min(self.source.len());
        let end = span.end.min(self.source.len());
        self.source[start..end].to_string()
    }

    ///This gets the entire line of code where the byte pos is located
    pub fn get_line_snippet(&self, byte_pos: usize) -> String {
        let (line_idx, _) = self.get_line_col(byte_pos);
        let line_start = self.line_starts[line_idx - 1];
        let line_end = if line_idx < self.line_starts.len() {
            self.line_starts[line_idx] - 1 //This excludes the /n as its not part of the line
        } else {
            self.source.len()
        };
        self.source[line_start..line_end].to_string()
    }
}
