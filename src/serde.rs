#![cfg(feature = "serde")]

extern crate serde;
use self::serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error, Unexpected, VariantAccess,
    Visitor,
};
use self::serde::ser::{Serialize, Serializer};

use {Level, LevelFilter, LOG_LEVEL_NAMES};

use std::fmt;
use std::str::{self, FromStr};

// The Deserialize impls are handwritten to be case insensitive using FromStr.

impl Serialize for Level {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Level::Output => serializer.serialize_unit_variant("Level", 1, "OUTPUT"),
            Level::Error => serializer.serialize_unit_variant("Level", 2, "ERROR"),
            Level::Warn => serializer.serialize_unit_variant("Level", 3, "WARN"),
            Level::Info => serializer.serialize_unit_variant("Level", 4, "INFO"),
            Level::Debug => serializer.serialize_unit_variant("Level", 5, "DEBUG"),
            Level::Trace => serializer.serialize_unit_variant("Level", 6, "TRACE"),
            Level::Detail => serializer.serialize_unit_variant("Level", 7, "DETAIL"),
            Level::Shell => serializer.serialize_unit_variant("Level", 8, "SHELL"),
        }
    }
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LevelIdentifier;

        impl<'de> Visitor<'de> for LevelIdentifier {
            type Value = Level;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("log level")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                // Case insensitive.
                FromStr::from_str(s).map_err(|_| Error::unknown_variant(s, &LOG_LEVEL_NAMES[1..]))
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let variant = str::from_utf8(value)
                    .map_err(|_| Error::invalid_value(Unexpected::Bytes(value), &self))?;

                self.visit_str(variant)
            }
        }

        impl<'de> DeserializeSeed<'de> for LevelIdentifier {
            type Value = Level;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_identifier(LevelIdentifier)
            }
        }

        struct LevelEnum;

        impl<'de> Visitor<'de> for LevelEnum {
            type Value = Level;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("log level")
            }

            fn visit_enum<A>(self, value: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (level, variant) = value.variant_seed(LevelIdentifier)?;
                // Every variant is a unit variant.
                variant.unit_variant()?;
                Ok(level)
            }
        }

        deserializer.deserialize_enum("Level", &LOG_LEVEL_NAMES[1..], LevelEnum)
    }
}

impl Serialize for LevelFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            LevelFilter::Off => serializer.serialize_unit_variant("LevelFilter", 0, "OFF"),
            LevelFilter::Output => serializer.serialize_unit_variant("LevelFilter", 1, "ERROR"),
            LevelFilter::Error => serializer.serialize_unit_variant("LevelFilter", 2, "ERROR"),
            LevelFilter::Warn => serializer.serialize_unit_variant("LevelFilter", 3, "WARN"),
            LevelFilter::Info => serializer.serialize_unit_variant("LevelFilter", 4, "INFO"),
            LevelFilter::Debug => serializer.serialize_unit_variant("LevelFilter", 5, "DEBUG"),
            LevelFilter::Trace => serializer.serialize_unit_variant("LevelFilter", 6, "TRACE"),
            LevelFilter::Detail => serializer.serialize_unit_variant("LevelFilter", 7, "DETAIL"),
            LevelFilter::Shell => serializer.serialize_unit_variant("LevelFilter", 8, "SHELL"),
        }
    }
}

impl<'de> Deserialize<'de> for LevelFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LevelFilterIdentifier;

        impl<'de> Visitor<'de> for LevelFilterIdentifier {
            type Value = LevelFilter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("log level filter")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                // Case insensitive.
                FromStr::from_str(s).map_err(|_| Error::unknown_variant(s, &LOG_LEVEL_NAMES))
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let variant = str::from_utf8(value)
                    .map_err(|_| Error::invalid_value(Unexpected::Bytes(value), &self))?;

                self.visit_str(variant)
            }
        }

        impl<'de> DeserializeSeed<'de> for LevelFilterIdentifier {
            type Value = LevelFilter;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_identifier(LevelFilterIdentifier)
            }
        }

        struct LevelFilterEnum;

        impl<'de> Visitor<'de> for LevelFilterEnum {
            type Value = LevelFilter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("log level filter")
            }

            fn visit_enum<A>(self, value: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (level_filter, variant) = value.variant_seed(LevelFilterIdentifier)?;
                // Every variant is a unit variant.
                variant.unit_variant()?;
                Ok(level_filter)
            }
        }

        deserializer.deserialize_enum("LevelFilter", &LOG_LEVEL_NAMES, LevelFilterEnum)
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;
    use self::serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};

    use {Level, LevelFilter};

    fn level_token(variant: &'static str) -> Token {
        Token::UnitVariant {
            name: "Level",
            variant: variant,
        }
    }

    fn level_bytes_tokens(variant: &'static [u8]) -> [Token; 3] {
        [
            Token::Enum { name: "Level" },
            Token::Bytes(variant),
            Token::Unit,
        ]
    }

    fn level_filter_token(variant: &'static str) -> Token {
        Token::UnitVariant {
            name: "LevelFilter",
            variant: variant,
        }
    }

    fn level_filter_bytes_tokens(variant: &'static [u8]) -> [Token; 3] {
        [
            Token::Enum {
                name: "LevelFilter",
            },
            Token::Bytes(variant),
            Token::Unit,
        ]
    }

    #[test]
    fn test_level_ser_de() {
        let cases = [
            (Level::Output, [level_token("OUTPUT")]),
            (Level::Error, [level_token("ERROR")]),
            (Level::Warn, [level_token("WARN")]),
            (Level::Info, [level_token("INFO")]),
            (Level::Debug, [level_token("DEBUG")]),
            (Level::Trace, [level_token("TRACE")]),
            (Level::Detail, [level_token("DETAIL")]),
            (Level::Shell, [level_token("SHELL")]),
        ];

        for &(s, expected) in &cases {
            assert_tokens(&s, &expected);
        }
    }

    #[test]
    fn test_level_case_insensitive() {
        let cases = [
            (Level::Output, [level_token("output")]),
            (Level::Error, [level_token("error")]),
            (Level::Warn, [level_token("warn")]),
            (Level::Info, [level_token("info")]),
            (Level::Debug, [level_token("debug")]),
            (Level::Trace, [level_token("trace")]),
            (Level::Detail, [level_token("detail")]),
            (Level::Shell, [level_token("shell")]),
        ];

        for &(s, expected) in &cases {
            assert_de_tokens(&s, &expected);
        }
    }

    #[test]
    fn test_level_de_bytes() {
        let cases = [
            (Level::Output, level_bytes_tokens(b"OUTPUT")),
            (Level::Error, level_bytes_tokens(b"ERROR")),
            (Level::Warn, level_bytes_tokens(b"WARN")),
            (Level::Info, level_bytes_tokens(b"INFO")),
            (Level::Debug, level_bytes_tokens(b"DEBUG")),
            (Level::Trace, level_bytes_tokens(b"TRACE")),
            (Level::Detail, level_bytes_tokens(b"DETAIL")),
            (Level::Shell, level_bytes_tokens(b"SHELL")),
        ];

        for &(value, tokens) in &cases {
            assert_de_tokens(&value, &tokens);
        }
    }

    #[test]
    fn test_level_de_error() {
        let msg = "unknown variant `errorx`, expected one of \
                   `OUTPUT`, `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`, `DETAIL`, `SHELL`";
        assert_de_tokens_error::<Level>(&[level_token("errorx")], msg);
    }

    #[test]
    fn test_level_filter_ser_de() {
        let cases = [
            (LevelFilter::Off, [level_filter_token("OFF")]),
            (LevelFilter::Output, [level_filter_token("OUTPUT")]),
            (LevelFilter::Error, [level_filter_token("ERROR")]),
            (LevelFilter::Warn, [level_filter_token("WARN")]),
            (LevelFilter::Info, [level_filter_token("INFO")]),
            (LevelFilter::Debug, [level_filter_token("DEBUG")]),
            (LevelFilter::Trace, [level_filter_token("TRACE")]),
            (LevelFilter::Detail, [level_filter_token("DETAIL")]),
            (LevelFilter::Shell, [level_filter_token("SHELL")]),
        ];

        for &(s, expected) in &cases {
            assert_tokens(&s, &expected);
        }
    }

    #[test]
    fn test_level_filter_case_insensitive() {
        let cases = [
            (LevelFilter::Off, [level_filter_token("off")]),
            (LevelFilter::Output, [level_filter_token("output")]),
            (LevelFilter::Error, [level_filter_token("error")]),
            (LevelFilter::Warn, [level_filter_token("warn")]),
            (LevelFilter::Info, [level_filter_token("info")]),
            (LevelFilter::Debug, [level_filter_token("debug")]),
            (LevelFilter::Trace, [level_filter_token("trace")]),
            (LevelFilter::Detail, [level_filter_token("detail")]),
            (LevelFilter::Shell, [level_filter_token("shell")]),
        ];

        for &(s, expected) in &cases {
            assert_de_tokens(&s, &expected);
        }
    }

    #[test]
    fn test_level_filter_de_bytes() {
        let cases = [
            (LevelFilter::Off, level_filter_bytes_tokens(b"OFF")),
            (LevelFilter::Output, level_filter_bytes_tokens(b"OUTPUT")),
            (LevelFilter::Error, level_filter_bytes_tokens(b"ERROR")),
            (LevelFilter::Warn, level_filter_bytes_tokens(b"WARN")),
            (LevelFilter::Info, level_filter_bytes_tokens(b"INFO")),
            (LevelFilter::Debug, level_filter_bytes_tokens(b"DEBUG")),
            (LevelFilter::Trace, level_filter_bytes_tokens(b"TRACE")),
            (LevelFilter::Detail, level_filter_bytes_tokens(b"DETAIL")),
            (LevelFilter::Shell, level_filter_bytes_tokens(b"SHELL")),
        ];

        for &(value, tokens) in &cases {
            assert_de_tokens(&value, &tokens);
        }
    }

    #[test]
    fn test_level_filter_de_error() {
        let msg = "unknown variant `errorx`, expected one of \
                   `OFF`, `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`";
        assert_de_tokens_error::<LevelFilter>(&[level_filter_token("errorx")], msg);
    }
}
