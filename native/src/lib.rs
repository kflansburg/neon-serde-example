extern crate serde;
extern crate serde_json;

use std::fmt;

use serde::de::{Visitor, Deserializer};
use neon::prelude::*;
use neon_serde;

#[macro_use]
extern crate serde_derive;
use serde::{Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Data {
    name: Name,
    index: usize
}

#[derive(Serialize, Deserialize)]
struct Link {
    page: Page,
    group: Group,
    stuff: Data
}

#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Name(usize);

#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Group(usize);

#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Page(usize);

struct GroupVisitor;

impl<'de> Visitor<'de> for GroupVisitor {
    type Value = Group;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an group name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Group(value[5..].parse().unwrap()))
    }
}

impl<'de> Deserialize<'de> for Group {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(GroupVisitor)
    }
}

struct PageVisitor;

impl<'de> Visitor<'de> for PageVisitor {
    type Value = Page;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an page name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Page(value[4..].parse().unwrap()))
    }
}

impl<'de> Deserialize<'de> for Page {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PageVisitor)
    }
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Name(value[4..].parse().unwrap()))
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NameVisitor)
    }
}

#[derive(Deserialize)]
struct ObjectTuple<K,V>(#[serde(with = "tuple_vec_map")] Vec<(K,V)>) where K: serde::de::DeserializeOwned, V: serde::de::DeserializeOwned;

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0: String = cx.argument::<JsString>(0)?.value();

    let object : ObjectTuple<Page, ObjectTuple<Group, Vec<Data>>> = serde_json::from_str(& arg0).unwrap();

    let list: Vec<Link> = object.0.iter().map(|(page_key, page)| {
        page.0.iter().map(move |(group_key, group)| {
           group.iter().map(move |data| {
               Link {
                   stuff: *data,
                   page: *page_key,
                   group: *group_key
               }
           })
        })
    }).flatten().flatten().collect();

    // return back to nodejs
    let result = neon_serde::to_value(&mut cx, &list)?;
    Ok(result)
    // Ok(cx.undefined().upcast())
}

register_module!(mut cx, { cx.export_function("hello", hello) });
