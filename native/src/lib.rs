#![feature(test)]
extern crate test;

use std::fs;

extern crate serde;
extern crate serde_json;

use std::fmt;

use serde::de::{Visitor, Deserializer};
use neon::prelude::*;
use neon_serde;

#[macro_use]
extern crate serde_derive;
use serde::{Deserialize};

#[derive(Serialize, Deserialize)]
struct Data {
    name: Name,
    index: usize
}

#[derive(Serialize, Deserialize)]
struct Link {
    page: usize,
    group: usize,
    stuff: serde_json::Value 
}

#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq)]
struct Name(usize);

#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq)]
struct Group(usize);

#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq)]
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

fn parse(input: &str) -> Vec<Link> {
    let mut object : ObjectTuple<Page, ObjectTuple<Group, Vec<serde_json::Value>>> = serde_json::from_str(input).unwrap();

    let mut list: Vec<Link> = Vec::new();

    for (Page(page), mut pages) in object.0.drain(..) {
        for (Group(group), mut groups)  in pages.0.drain(..) {
            for stuff in groups.drain(..) {
                list.push(Link {
                    stuff,
                    page,
                    group
                })
            }
        }
    }

    list
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0: String = cx.argument::<JsString>(0)?.value();
    let result = neon_serde::to_value(&mut cx, &parse(&arg0))?;
    Ok(result)
}

register_module!(mut cx, { cx.export_function("hello", hello) });

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let input = fs::read_to_string("input.json").unwrap(); 
        b.iter(|| parse(&input));
    }
}
