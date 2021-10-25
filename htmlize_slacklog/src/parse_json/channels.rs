use serde_json::{Result, Value};

pub struct channel {
    id: String,
    name: String,
}

impl channel {

}

#[test]
fn slacklog_json_parse_test() {
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

}

