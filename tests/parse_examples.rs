use factorio_blueprint::{BlueprintCodec, Container};
use std::io::{BufReader, Read};
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

fn container_examples() -> impl Iterator<Item = Container> {
    examples().map(|path| {
        BlueprintCodec::decode(BufReader::new(
            std::fs::File::open(path).expect("can open example file"),
        ))
        .expect("container decoded successfully")
    })
}

fn test_parse<R: Read>(reader: R) {
    let mut json_data = Vec::new();
    BlueprintCodec::decode_reader(reader, |mut reader| {
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

#[test]
fn can_parse_examples() {
    for example in examples() {
        let file_data = std::fs::read_to_string(example).expect("can read from file");

        test_parse(file_data.trim().as_bytes());
    }
}

#[test]
fn can_read_from_untrimmed_files() {
    for example in examples() {
        test_parse(BufReader::new(
            std::fs::File::open(example).expect("can open example file"),
        ));
    }
}

#[test]
fn roundtrip() {
    for container in container_examples() {
        let mut blueprint = Vec::new();
        container
            .encode(&mut blueprint)
            .expect("encoding should succeed");
        let roundtripped = Container::decode(&blueprint as &[u8]).expect("decoding should succeed");
        assert_eq!(container, roundtripped);
    }
}
