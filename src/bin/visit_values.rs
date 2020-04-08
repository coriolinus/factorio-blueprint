use factorio_blueprint::{objwalk::Objwalk, BlueprintCodec};
use serde_json::Value as JsonValue;

fn main() {
    // target/debug/1/executable
    let mut examples = std::env::current_exe().expect("can find test executable");
    for _ in 0..3 {
        examples.pop();
    }
    examples.push("tests/examples");
    dbg!(&examples);

    for maybe_example in std::fs::read_dir(examples).expect("should find examples dir") {
        let example = maybe_example.expect("should find file").path();
        let file_data = std::fs::read_to_string(example).expect("can read from file");

        let container =
            BlueprintCodec::decode(file_data.trim().as_bytes()).expect("can parse as container");
        container.walk_structure(|value| {
            if let Some(value) = value.downcast_ref::<JsonValue>() {
                println!(
                    "{}",
                    serde_json::to_string(&value).expect("can reserialize")
                );
            }
        })
    }
}
