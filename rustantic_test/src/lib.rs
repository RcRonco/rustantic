#![allow(dead_code)]
use pyo3::prelude::*;
use rustantic_macros::pydantic;
use uuid::Uuid;

#[pydantic]
#[derive(Clone, PartialEq)]
pub struct Nested {
    pub name: String,
    pub num: u32,
    pub id: Uuid,
}

#[pymethods]
impl Nested {
    #[new]
    pub fn new(name: String, num: u32, id: Uuid) -> PyResult<Self> {
        Ok(Self { name, num, id: id })
    }
}
#[pydantic]
#[derive(Clone, PartialEq)]
pub struct Nested2 {
    pub name: String,
    pub num: u32,
}

#[pydantic]
#[derive(Clone)]
enum MyEnum {
    A(Nested),
    B(Nested),
    C(i16),
    D(),
}

#[pyfunction]
fn check_my_enum(f: MyEnum) -> String {
    match f {
        MyEnum::A(_) => "A".to_string(),
        MyEnum::B(_) => "B".to_string(),
        MyEnum::C(_) => "C".to_string(),
        MyEnum::D() => "D".to_string(),
    }
}


#[pydantic]
#[derive(Clone)]
enum MyUnitEnum {
    A,
    B = 300,
    C = 900,
    D,
}

#[pyfunction]
fn check_my_unit_enum(f: MyUnitEnum) -> String {
    match f {
        MyUnitEnum::A => "A".to_string(),
        MyUnitEnum::B => "B".to_string(),
        MyUnitEnum::C => "C".to_string(),
        MyUnitEnum::D => "D".to_string(),
    }
}

#[pydantic(aaa, bbb)]
#[derive(Clone)]
struct MyClass {
    pub name: String,
    pub num: u32,
    pub vec: Vec<f32>,
    pub nested: Nested,
    pub myenum: MyUnitEnum,
}

#[pymethods]
impl MyClass {
    #[new]
    #[pyo3(signature = (name, num2, vec, nested, myenum, _nesteds, _opt1=None, _opt2=None, _opt3=None))]
    pub fn new(
        name: String,
        num2: u32,
        vec: Vec<f32>,
        nested: Nested,
        myenum: MyUnitEnum,
        _nesteds: Vec<Nested>,
        _opt1: Option<i32>,
        _opt2: Option<Nested>,
        _opt3: Option<Vec<Nested>>,
    ) -> PyResult<Self> {
        Ok(Self {
            name,
            num: num2,
            vec,
            nested,
            myenum,
        })
    }
}


#[pymodule]
fn rustantic_test(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Nested>()?;
    m.add_class::<MyClass>()?;
    m.add_class::<MyUnitEnum>()?;
    m.add_class::<MyEnum>()?;

    m.add_function(wrap_pyfunction!(check_my_enum, m)?)?;
    m.add_function(wrap_pyfunction!(check_my_unit_enum, m)?)?;

    Ok(())
}
