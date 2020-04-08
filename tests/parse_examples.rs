use factorio_blueprint::{BlueprintCodec, Container};
use std::io::{BufReader, Read};

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
        let file_data = std::fs::read_to_string(example).expect("can read from file");

        let mut json_data = Vec::new();
        BlueprintCodec::decode_reader(file_data.trim().as_bytes(), |mut reader| {
            reader.read_to_end(&mut json_data).map(|_| ())
        })
        .expect("should have expected version");
        let json_str = String::from_utf8(json_data).expect("json data should be valid utf-8");

        if let Err(jserr) = serde_json::from_str::<Container>(&json_str) {
            use serde_json::error::Category;
            match jserr.classify() {
                Category::Syntax | Category::Data => {
                    const SPACING: usize = 50;
                    let mut col = jserr.column();
                    col -= col.min(SPACING);
                    println!("bad json: {}", &json_str[col..col + 50]);
                }
                _ => {}
            }
            panic!("error: {}", jserr);
        }
    }
}

#[test]
fn can_read_from_untrimmed_files() {
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

        let mut json_data = Vec::new();
        BlueprintCodec::decode_reader(
            BufReader::new(std::fs::File::open(example).expect("can open example file")),
            |mut reader| reader.read_to_end(&mut json_data).map(|_| ()),
        )
        .expect("should have expected version");
        let json_str = String::from_utf8(json_data).expect("json data should be valid utf-8");

        if let Err(jserr) = serde_json::from_str::<Container>(&json_str) {
            use serde_json::error::Category;
            match jserr.classify() {
                Category::Syntax | Category::Data => {
                    const SPACING: usize = 50;
                    let mut col = jserr.column();
                    col -= col.min(SPACING);
                    println!("bad json: {}", &json_str[col..col + 50]);
                }
                _ => {}
            }
            panic!("error: {}", jserr);
        }
    }
}
