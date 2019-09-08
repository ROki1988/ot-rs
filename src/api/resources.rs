use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ResourceName(String);

impl ResourceName {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for ResourceName {
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
pub struct ResourceValue(String);

impl ResourceValue {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for ResourceValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 255 && s.chars().all(|x| x >= ' ' && x <= '~') {
            Ok(Self(s.to_string()))
        } else {
            Err(())
        }
    }
}

pub struct Resource(HashMap<ResourceName, ResourceValue>);

impl Resource {
    fn upsert(&mut self, name: ResourceName, value: ResourceValue) -> &mut Self {
        self.0.insert(name, value);
        self
    }

    fn try_upsert(&mut self, name: &str, value: &str) -> Result<&mut Self, ()> {
        ResourceName::from_str(name).and_then(|n| {
            ResourceValue::from_str(value).map(|v| {
                self.0.insert(n, v);
                self
            })
        })
    }

    fn merge(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }
}
