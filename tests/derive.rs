#[macro_use]
extern crate trackable_derive;

#[test]
fn it_works() {
    #[derive(TrackableError)]
    pub struct Error;
}
