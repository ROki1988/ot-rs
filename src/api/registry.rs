use crate::api::unit::Unit;

pub struct Variable {
    name: String,
    description: String,
    unit: Unit,
    // FIXME: modify name to type
    type_: String,
}
