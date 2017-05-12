mod inner {
    #[derive(Debug)]
    pub struct SomeStruct {
        pub hello: &'static str,
        pub thank: &'static str,
    }
}


fn main() {
    let some_struct = SomeStruct {
        hello: "from missing_import",
        thank: "you for fixing me!"
    };
    println!("{:#?}", some_struct);
}
