use std::convert::TryFrom;

use bytes::Bytes;

pub trait TryFromHttpText: Sized {
    type Err;
    fn try_from_http_text(s: &str) -> Result<Self, Self::Err>;
}

pub trait ToHttpText {
    fn to_http_text(&self) -> String;
}

pub trait BinaryFormat<T>: TryFrom<Bytes> {
    fn to_bytes(&self) -> Bytes;
}

pub trait HttpTextFormat {
    fn fields(&self) -> &[&str];
}

pub trait HttpTextInject: HttpTextFormat + ToHttpText {
    fn inject<C, R>(&self, carrier: &mut C, setter: fn(&mut C, String, String) -> R);
}

pub trait HttpTextExtract: HttpTextFormat + TryFromHttpText {
    // TODO: use result
    fn extract<C>(
        carrier: &C,
        getter: for<'r> fn(&'r C, &str) -> Option<&'r String>,
    ) -> Option<Self>;
}
