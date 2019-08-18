use core::num::{NonZeroU128, NonZeroU64};
use std::str::FromStr;

use rand;

use bitflags::bitflags;

#[derive(Debug)]
pub struct TraceId(NonZeroU128);

impl TraceId {
    pub const fn size() -> usize {
        std::mem::size_of::<u128>()
    }

    pub fn new(id: NonZeroU128) -> Self {
        Self(id)
    }

    pub fn generate_random() -> Self {
        Self::new(rand::random::<NonZeroU128>())
    }

    pub fn to_base16(&self) -> String {
        format!("{:032x}", self.0.get().to_be())
    }

    // TODO: move to TryFrom
    pub fn try_from_base16(value: &str) -> Option<Self> {
        u128::from_str_radix(value, 16)
            .map(u128::from_be)
            .ok()
            .and_then(NonZeroU128::new)
            .map(Self::new)
    }
}

impl PartialEq for TraceId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for TraceId {}

impl ToString for TraceId {
    fn to_string(&self) -> String {
        format!("{:032x}", self.0)
    }
}

#[derive(Debug)]
pub struct SpanId(NonZeroU64);

impl SpanId {
    pub const fn size() -> usize {
        std::mem::size_of::<u64>()
    }

    pub fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    pub fn generate_random() -> Self {
        Self::new(rand::random::<NonZeroU64>())
    }

    pub fn to_base16(&self) -> String {
        format!("{:016x}", self.0.get().to_be())
    }

    pub fn try_from_base16(value: &str) -> Option<Self> {
        u64::from_str_radix(value, 16)
            .map(u64::from_be)
            .ok()
            .and_then(NonZeroU64::new)
            .map(Self::new)
    }
}

impl PartialEq for SpanId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for SpanId {}

impl ToString for SpanId {
    fn to_string(&self) -> String {
        format!("{:016x}", self.0)
    }
}

bitflags! {
    pub struct TraceOption: u8 {
        const MASK_SAMPLE = 0x01;
        const MASK_UNUSED = 0xFE;
    }
}

impl TraceOption {
    pub const fn size() -> usize {
        std::mem::size_of::<u8>()
    }

    pub fn to_base16(self) -> String {
        format!("{:02x}", self.bits.to_be())
    }

    pub fn try_from_base16(value: &str) -> Option<Self> {
        u8::from_str(value).ok().and_then(Self::from_bits)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Key(String);

impl Key {
    fn try_from(value: String) -> Option<Self> {
        let s = value.trim();
        if s.is_empty() || s.len() > 256 {
            return None;
        };
        s.chars().next().filter(|x| x.is_ascii_lowercase())?;
        let is_valid = |x: char| {
            x.is_ascii_lowercase() || x.is_ascii_digit() || ['_', '-', '*', '/'].contains(&x)
        };
        if !s.chars().all(is_valid) {
            return None;
        };

        Some(Self(s.to_owned()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Value(String);
impl Value {
    fn try_from(value: String) -> Option<Self> {
        let s = value.trim().to_owned();
        if s.is_empty() || s.len() > 256 {
            return None;
        };
        if !s
            .chars()
            .all(|x| x.is_ascii_graphic() && x != ',' && x != '=')
        {
            return None;
        };

        Some(Self(s))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    key: Key,
    value: Value,
}

impl Entry {
    pub fn try_from(key: String, value: String) -> Option<Self> {
        Key::try_from(key).and_then(|k| Value::try_from(value).map(|v| Self { key: k, value: v }))
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        format!("{}={}", self.key.0, self.value.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceState(pub(crate) Vec<Entry>);

impl TraceState {
    pub(crate) const fn max_entry_size() -> usize {
        32
    }
    pub fn propagate(parent: &Self) -> Self {
        Self(parent.0.to_vec())
    }

    pub fn empty() -> Self {
        Self(Vec::with_capacity(Self::max_entry_size()))
    }

    pub fn upsert(&mut self, entry: Entry) {
        if let Some(c) = self.0.iter_mut().find(|x| x.key == entry.key) {
            c.value = entry.value;
        } else if self.0.len() < Self::max_entry_size() {
            self.0.push(entry);
        } else {
            // TODO: over limit pattern
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.0.iter()
    }

    pub fn has_entry(&self) -> bool {
        !self.0.is_empty()
    }
}

#[derive(Debug)]
pub struct SpanContext<'a> {
    pub trace_id: &'a TraceId,
    pub span_id: SpanId,
    pub trace_option: TraceOption,
    pub trace_state: TraceState,
}

impl<'a> PartialEq for SpanContext<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.trace_id == other.trace_id
            && self.span_id == other.span_id
            && self.trace_option.bits() == other.trace_option.bits()
    }
}

impl<'a> SpanContext<'a> {
    pub fn new(
        trace_id: &'a TraceId,
        span_id: SpanId,
        trace_option: TraceOption,
        trace_state: TraceState,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            trace_option,
            trace_state,
        }
    }

    pub fn span_id_str(&self) -> String {
        self.span_id.to_string()
    }

    pub fn trace_id_str(&self) -> String {
        self.trace_id.to_string()
    }

    pub fn is_sample(&self) -> bool {
        self.trace_option.contains(TraceOption::MASK_SAMPLE)
    }
}
