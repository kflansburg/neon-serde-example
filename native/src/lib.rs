extern crate serde;
extern crate serde_json;

use neon::prelude::*;

#[macro_use]
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;

#[derive(Serialize, Deserialize)]
struct Link {
    page_key: String,
    group_key: String,
    stuff: Value,
}

export! {
    fn hellojson() -> String {
        let jsondata = json!({"page1":{"group1":[{"name":"name1","index":1},{"name":"name33","index":33},{"name":"name45","index":45}]},"page500":{"group25":[{"name":"name1","index":1},{"name":"name33","index":33},{"name":"name45","index":45}]}});
        let result = serde_json::to_string(&jsondata).unwrap();
        return format!("{}", result);
    }
    fn hello2(input: String) -> String {
        let object: Value = serde_json::from_str(&input).unwrap();
        let result = serde_json::to_string(&object).unwrap();
        return format!("{}", result);
    }
    fn hello(input: String) -> String {
        let mut list = vec![];
        let object: Value = serde_json::from_str(&input).unwrap();
        for (page_key, page) in object.as_object().unwrap().iter() {
            for (group_key, group) in page.as_object().unwrap().iter() {
                for stuff in group.as_array().unwrap() {
                    let link = Link {
                        page_key: page_key.to_string(),
                        group_key: group_key.to_string(),
                        stuff: stuff.clone(),
                    };
                    list.push(link);
                }
            }
        }
        let result = serde_json::to_string(&list).unwrap();
        return format!("{}", result);
    }
}
