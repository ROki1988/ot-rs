pub enum Unit {
    Dimensionless,
    Bytes,
    Milliseconds,
}

impl Unit {
    fn to_str(&self) -> &str {
        match self {
            Unit::Dimensionless => "1",
            Unit::Bytes => "By",
            Unit::Milliseconds => "ms",
        }
    }
}
