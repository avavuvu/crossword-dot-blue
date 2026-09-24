use std::fmt;

use sea_orm::{
    ColIdx, QueryResult, TryGetError, TryGetable, Value,
    sea_query::{ArrayType, ColumnType, Nullable, StringLen, ValueType, ValueTypeErr},
};

pub const MAX_LEN: u32 = 16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Region {
    Au,
    Uk,
    Us,
    Custom(String),
}

impl Region {
    pub const DEFAULTS: [Region; 3] = [Region::Au, Region::Uk, Region::Us];

    pub fn as_str(&self) -> &str {
        match self {
            Region::Au => "AU",
            Region::Uk => "UK",
            Region::Us => "US",
            Region::Custom(value) => value,
        }
    }

    pub fn parse(value: &str) -> Option<Region> {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }

        Some(match value.to_ascii_uppercase().as_str() {
            "AU" => Region::Au,
            "UK" => Region::Uk,
            "US" => Region::Us,
            _ => Region::Custom(value.to_string()),
        })
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Region> for Value {
    fn from(region: Region) -> Self {
        Value::String(Some(Box::new(region.as_str().to_string())))
    }
}

impl TryGetable for Region {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: Option<String> = res.try_get_by(index)?;
        value
            .as_deref()
            .and_then(Region::parse)
            .ok_or_else(|| TryGetError::Null(format!("{index:?}")))
    }
}

impl ValueType for Region {
    fn try_from(value: Value) -> Result<Self, ValueTypeErr> {
        match value {
            Value::String(Some(text)) => Region::parse(&text).ok_or(ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Region".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::N(MAX_LEN))
    }
}

impl Nullable for Region {
    fn null() -> Value {
        Value::String(None)
    }
}
