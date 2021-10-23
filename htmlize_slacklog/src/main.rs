use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
{
    "id": "C01TD4HG17U",
    "name": "general",
    "created": 1617704898,
    "creator": "U01TDAQEZFV",
    "is_archived": false,
    "is_general": true,
    "members": [
        "U01TDAQEZFV",
        "U01TWHTPGEA",
        "U01U3HG4UCB",
        "U01U9G4EMK6"
    ],
    "topic": {
        "value": "<https:\/\/meet.google.com\/jyc-oeib-hbs?authuser=0>",
        "creator": "U01TDAQEZFV",
        "last_set": 1621829202
    },
    "purpose": {
        "value": "このチャンネルには、常にすべてのメンバーが含まれます。社内通知やチーム全体の会話にぴったりです。",
        "creator": "U01TDAQEZFV",
        "last_set": 1617704898
    }
}
        "#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    // println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("{:?}", v);
    Ok(())
}

fn main() {
    untyped_example().unwrap();
}
