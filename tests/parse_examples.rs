#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn examples() -> impl Iterator<Item = PathBuf> {
        let mut examples = std::env::current_exe().expect("can find test executable");
        // target/debug/1/executable
        for _ in 0..4 {
            examples.pop();
        }
        examples.push("tests/examples");

        std::fs::read_dir(examples)
            .expect("should find examples dir")
            .map(|maybe_example| maybe_example.expect("should find file").path())
            .inspect(|example| {
                dbg!(example);
            })
    }

    #[test]
    fn roundtrip() {
        for example in examples() {
            factorio_blueprint::roundtrip_blueprint_test(&std::fs::read_to_string(example).unwrap())
        }
    }
}
