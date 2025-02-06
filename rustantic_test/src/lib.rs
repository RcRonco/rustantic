use pyo3::prelude::*;
use rustantic_macros::pydantic;

#[pydantic]
#[derive(Clone, Debug, PartialEq)]
pub struct Nested {
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
#[derive(Clone, Debug, PartialEq)]
struct MyClass {
    pub name: String,
    pub num: u32,
    pub vec: Vec<f32>,
    pub nested: Nested,
}

#[pymethods]
impl MyClass {
    #[new]
    pub fn new(name: String, num2: u32, nested: Nested) -> PyResult<Self> {
        Ok(Self {
            name,
            num: num2,
            vec: vec![],
            nested,
        })
    }
}

#[pymodule]
fn rustantic_test(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Nested>()?;
    m.add_class::<MyClass>()?;

    Ok(())
}
