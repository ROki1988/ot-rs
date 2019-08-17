use std::convert::TryFrom;

use bytes::Bytes;

pub mod key;
pub mod span;
pub mod span_context;
pub mod status;
pub mod trace_context;

pub trait BinaryFormat<T>: TryFrom<Bytes> {
    fn to_bytes(&self) -> Bytes;
}

pub trait HttpTextFormat {
    type Item;
    fn fields(&self) -> &[&str];

    fn inject<C, R>(&self, carrier: &mut C, setter: fn(&mut C, String, String) -> R);

    // TODO: use result
    fn extract<C>(
        carrier: &C,
        getter: for<'r> fn(&'r C, &str) -> Option<&'r String>,
    ) -> Option<Self::Item>;
}
