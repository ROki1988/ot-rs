use ot_rs::api::trace::status::Status;

#[test]
fn it_works() {
    let a = Status::ok().with_description("test".to_owned());
}
