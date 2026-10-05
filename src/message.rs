// Released under the MIT License.
// Copyright, 2024-2025, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

use crate::unist::{Point, Position};
use alloc::{boxed::Box, fmt, string::String};

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    /// Place of message.
    pub place: Option<Box<Place>>,
    /// Reason for message (should use markdown).
    pub reason: String,
    /// Category of message.
    pub rule_id: Box<String>,
    /// Namespace of message.
    pub source: Box<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref place) = self.place {
            write!(f, "{place}: ")?;
        }

        write!(f, "{} ({}:{})", self.reason, self.source, self.rule_id)
    }
}

/// Somewhere.
#[derive(Clone, Debug, PartialEq)]
pub enum Place {
    /// Between two points.
    Position(Position),
    /// At a point.
    Point(Point),
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Place::Position(position) => write!(
                f,
                "{}:{}-{}:{}",
                position.start.line, position.start.column, position.end.line, position.end.column
            ),
            Place::Point(point) => write!(f, "{}:{}", point.line, point.column),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, Place};
    use crate::unist::{Point, Position};
    use alloc::boxed::Box;
    use alloc::string::ToString;
    use core::fmt::Write as _;

    struct FailingWriter;

    impl core::fmt::Write for FailingWriter {
        fn write_str(&mut self, _value: &str) -> core::fmt::Result {
            Err(core::fmt::Error)
        }
    }

    #[test]
    fn displays_a_message_with_its_place() {
        let message = Message {
            place: Some(Box::new(Place::Point(Point::new(2, 3, 4)))),
            reason: "invalid value".into(),
            rule_id: Box::new("example".into()),
            source: Box::new("parser".into()),
        };

        assert_eq!(message.to_string(), "2:3: invalid value (parser:example)");

        let message = Message {
            place: Some(Box::new(Place::Position(Position::new(1, 2, 3, 4, 5, 6)))),
            reason: "invalid range".into(),
            rule_id: Box::new("example".into()),
            source: Box::new("parser".into()),
        };

        assert_eq!(
            message.to_string(),
            "1:2-4:5: invalid range (parser:example)"
        );

        let message = Message {
            place: None,
            reason: "invalid without a location".into(),
            rule_id: Box::new("example".into()),
            source: Box::new("parser".into()),
        };
        assert_eq!(
            message.to_string(),
            "invalid without a location (parser:example)"
        );

        let mut writer = FailingWriter;
        assert!(write!(&mut writer, "{message}").is_err());

        let message = Message {
            place: Some(Box::new(Place::Point(Point::new(2, 3, 4)))),
            reason: "invalid value".into(),
            rule_id: Box::new("example".into()),
            source: Box::new("parser".into()),
        };
        assert!(write!(&mut writer, "{message}").is_err());
    }
}
