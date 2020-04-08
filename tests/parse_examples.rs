use factorio_blueprint::Container;

#[test]
fn can_parse_examples() {
    // target/debug/1/executable
    let mut examples = std::env::current_exe().expect("can find test executable");
    for _ in 0..4 {
        examples.pop();
    }
    examples.push("tests/examples");
    dbg!(&examples);

    for maybe_example in std::fs::read_dir(examples).expect("should find examples dir") {
        let example = maybe_example.expect("should find file").path();
        dbg!(example.file_name());
        let file = std::fs::File::open(example).expect("can open file");
        let read_result = dbg!(Container::read_blueprint(file));
        assert!(read_result.is_ok());
    }
}
