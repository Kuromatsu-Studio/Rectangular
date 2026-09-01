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

    fn get_line_starts(source: &str) -> Vec<usize> {
        let mut starts = vec![0];
        for (byte_pos, ch) in source.char_indices() {
            if ch == '\n' {
                starts.push(byte_pos + 1);
            }
        }
        starts
    }
}
