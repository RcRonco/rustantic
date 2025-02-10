#![allow(dead_code)]
use pyo3::prelude::*;
use rustantic_macros::pydantic;

#[pydantic]
#[derive(Clone, PartialEq)]
pub struct Nested {
    pub name: String,
    pub num: u32,
}

#[pydantic]
#[derive(Clone, PartialEq)]
pub struct Nested2 {
    pub name: String,
    pub num: u32,
}

#[pymethods]
impl Nested {
    #[new]
    pub fn new(name: String, num: u32) -> PyResult<Self> {
        Ok(Self { name, num })
    }
}

#[pydantic]
#[derive(Clone)]
enum MyEnum {
    A(Nested),
    B(Nested),
    C(i16),
}

#[pydantic]
#[derive(Clone)]
enum MyUnitEnum {
    A,
    B = 300,
    C = 900,
    D,
}

#[pydantic]
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
    pub fn new(name: String, num2: u32, nested: Nested, myenum: MyUnitEnum) -> PyResult<Self> {
        Ok(Self {
            name,
            num: num2,
            vec: vec![],
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

    Ok(())
}
