mod util;
use schemars::{JsonSchema, Map};
use util::*;

// Ensure that schemars_derive uses the full path to std::string::String
pub struct String;

#[derive(JsonSchema)]
struct UnitStruct;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct Struct {
    foo: i32,
    bar: bool,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(rename_all = "camelCase")]
enum External {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    UnitTwo,
    Tuple(i32, bool),
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_external_tag() -> TestResult {
    test_default_generated_schema::<External>("enum-external")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(tag = "typeProperty")]
enum Internal {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    UnitTwo,
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_internal_tag() -> TestResult {
    test_default_generated_schema::<Internal>("enum-internal")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(untagged)]
enum Untagged {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    Tuple(i32, bool),
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_untagged() -> TestResult {
    test_default_generated_schema::<Untagged>("enum-untagged")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(tag = "t", content = "c")]
enum Adjacent {
    UnitOne,
    StringMap(Map<&'static str, &'static str>),
    UnitStructNewType(UnitStruct),
    StructNewType(Struct),
    Struct {
        foo: i32,
        bar: bool,
    },
    Tuple(i32, bool),
    UnitTwo,
    #[schemars(with = "i32")]
    WithInt,
}

#[test]
fn enum_adjacent_tagged() -> TestResult {
    test_default_generated_schema::<Adjacent>("enum-adjacent-tagged")
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(tag = "typeProperty")]
enum SimpleInternal {
    A,
    B,
    C,
}

#[test]
fn enum_simple_internal_tag() -> TestResult {
    test_default_generated_schema::<SimpleInternal>("enum-simple-internal")
}
<<<<<<< HEAD

#[allow(dead_code)]
#[derive(JsonSchema)]
enum SoundOfMusic {
    /// # A deer
    ///
    /// A female deer
    Do,
    /// A drop of golden sun
    Re,
    /// A name I call myself
    Mi,
}

#[test]
fn enum_unit_with_doc_comments() -> TestResult {
    test_default_generated_schema::<SoundOfMusic>("enum-unit-doc")
}
||||||| e9d5f40
=======

#[derive(Debug, JsonSchema)]
pub enum Aliased {
    #[schemars(alias = "v1", alias = "variant1")]
    V1,
    #[schemars(alias = "v2", alias = "variant2")]
    V2,
}
#[test]
fn enum_aliased() -> TestResult {
    test_default_generated_schema::<Aliased>("enum-aliased")
}
>>>>>>> 8024f2986554cb10877dc21acdd39dc9037e8b8d
