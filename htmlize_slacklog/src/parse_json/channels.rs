use serde_json::{Result, Value};

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Channels {
    id: String,
    name: String,
}

impl Channels {
    fn new(v: Value) -> Self {
        Channels {
            id: v["id"].to_string().chars().filter(|s| !['\"'].contains(&s)).collect(),
            name: v["name"].to_string().chars().filter(|s| !['\"'].contains(&s)).collect(),
        }
    }
}

#[test]
fn channel_short_log_json_parse_test() {
    let data = r#"
{
    "id": "C01TD4HG17U",
    "name": "general"
}
        "#;
    let v: Value = serde_json::from_str(data).unwrap();
    let channel = Channels::new(v);

    let ans: Channels = Channels{id: "C01TD4HG17U".to_string(), name: "general".to_string()};
    assert_eq!(ans, channel);
}

#[test]
fn channel_long_log_json_parse_test() {
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
    let v: Value = serde_json::from_str(data).unwrap();
    let channel = Channels::new(v);

    let ans: Channels = Channels{id: "C01TD4HG17U".to_string(), name: "general".to_string()};
    assert_eq!(ans, channel);
}

