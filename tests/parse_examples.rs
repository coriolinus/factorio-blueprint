use factorio_blueprint::{BlueprintCodec, Container, Error};
use std::io::{BufReader, Read, Write};
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

/// Decodes a blueprint string into the serde_json Value
fn decode_to_json_value<R: std::io::Read>(reader: R) -> Result<serde_json::value::Value, Error> {
    let mut out = Err(Error::NoData);
    BlueprintCodec::decode_reader(reader, |reader| {
        out = serde_json::from_reader(reader).map_err(|e| e.into());
        Ok(())
    })?;
    out
}

/// Decodes a blueprint string and compares that parsed json matches
/// after we decode and then re-encode it.
fn roundtrip_blueprint_test(blueprint: &str) {
    let bp2 =
        BlueprintCodec::encode_string(&BlueprintCodec::decode_string(blueprint).unwrap()).unwrap();
    let json_v1 = decode_to_json_value(blueprint.as_bytes()).unwrap();
    let json_v2 = decode_to_json_value(bp2.as_bytes()).unwrap();

    if json_v1 != json_v2 {
        let mut w = std::fs::File::create("output1.json").unwrap();
        write!(w, "{:#}", json_v1).unwrap();
        
        let mut w = std::fs::File::create("output2.json").unwrap();
        write!(w, "{:#}", json_v2).unwrap();
        panic!("Mismatched output found, compare output of output1.json and output2.json");
    }
}


#[test]
fn roundtrip_new() {
    for example in examples() {
        roundtrip_blueprint_test(&std::fs::read_to_string(example).unwrap())
    }
}
