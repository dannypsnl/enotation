#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticSpan {
    pub start_line: usize,
    pub start_col: usize,
    pub start_offset: usize,

    pub end_line: usize,
    pub end_col: usize,
    pub end_offset: usize,

    pub span: String,
}

impl Default for DiagnosticSpan {
    fn default() -> Self {
        Self {
            start_line: 0,
            start_col: 0,
            start_offset: 0,
            end_line: 0,
            end_col: 0,
            end_offset: 0,
            span: String::new(),
        }
    }
}

impl DiagnosticSpan {
    pub fn from_pest_span(span: &pest::Span<'_>) -> Self {
        let (start_line, start_col) = span.start_pos().line_col();
        let (end_line, end_col) = span.end_pos().line_col();
        let start_offset = span.start();
        let end_offset = span.end();
        let source = span.as_str().to_string();
        Self {
            start_line,
            start_col,
            start_offset,
            end_line,
            end_col,
            end_offset,
            span: source,
        }
    }
}
