use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct LabelName(String);

impl LabelName {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for LabelName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() && s.len() <= 255 && s.chars().all(|x| x >= ' ' && x <= '~') {
            Ok(Self(s.to_string()))
        } else {
            Err(())
        }
    }
}

#[derive(Clone)]
pub struct LabelValue(String);

impl LabelValue {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for LabelValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 255 && s.chars().all(|x| x >= ' ' && x <= '~') {
            Ok(Self(s.to_string()))
        } else {
            Err(())
        }
    }
}

pub struct Resource(HashMap<LabelName, LabelValue>);

impl Resource {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn upsert(&mut self, name: LabelName, value: LabelValue) -> &mut Self {
        self.0.insert(name, value);
        self
    }

    pub fn try_upsert(&mut self, name: &str, value: &str) -> Result<&mut Self, ()> {
        LabelName::from_str(name).and_then(|n| {
            LabelValue::from_str(value).map(|v| {
                self.0.insert(n, v);
                self
            })
        })
    }

    pub fn merge(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    pub fn labels(&self) -> impl Iterator<Item = (&LabelName, &LabelValue)> {
        self.0.iter()
    }
}
