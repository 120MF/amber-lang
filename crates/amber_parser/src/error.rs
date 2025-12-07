use miette::{Diagnostic, NamedSource, SourceOffset, SourceSpan};
use pest::error::{Error as PestError, InputLocation};
use thiserror::Error;

use crate::Rule;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("I/O error while reading source")]
    #[diagnostic(code(amber_parser::io_error))]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("parse error in {name}: {source}")]
    #[diagnostic(code(amber_parser::parse_error))]
    Pest {
        name: String,
        source: PestError<Rule>,
        #[source_code]
        src: NamedSource<String>,
        #[label("around here")]
        span: SourceSpan,
    },
}

impl ParseError {
    pub fn from_pest(
        source: PestError<Rule>,
        name: impl AsRef<str>,
        input: impl Into<String>,
    ) -> Self {
        let span_location = source.location.clone();
        let name = name.as_ref().to_string();
        ParseError::Pest {
            name: name.clone(),
            source,
            src: NamedSource::new(name.clone(), input.into()),
            span: location_to_span(span_location),
        }
    }
}

fn location_to_span(location: InputLocation) -> SourceSpan {
    match location {
        InputLocation::Pos(pos) => SourceSpan::new((pos).into(), 0),
        InputLocation::Span((start, end)) => {
            let len = end.saturating_sub(start);
            SourceSpan::new(SourceOffset::from(start), len)
        }
    }
}
