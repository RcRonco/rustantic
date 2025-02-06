# Rustantic - Pydantic Generated from Rust
Rustantic is a Rust-based code generation tool that automatically generates Pydantic models for Python from Rust structs annotated with custom attributes. 
It leverages [PyO3](https://pyo3.rs/) to expose Rust types and functions to Python, while using a build script to scan your source code and generate matching Pydantic models.  
The generated Python models include a `to_rs()` method that creates a PyO3-wrapped Rust instance using custom constructors (detected via the `#[new]` attribute).

## Features

- **Automatic Code Generation:**  
  Annotate your Rust structs with `#[pydantic]` and have the corresponding Pydantic models generated at build time.

- **PyO3 Integration:**  
  Automatically inject the `#[pyclass]` attribute into your Rust types, exposing them to Python.

- **Custom Constructor Detection:**  
  Detect custom PyO3 constructors marked with `#[new]` and generate appropriate `to_rs()` methods in the Python models.

- **Build-Time Generation:**  
  Uses a build script (with the help of [walkdir](https://crates.io/crates/walkdir) and [syn](https://docs.rs/syn/)) to scan your entire `src/` directory, ensuring that all relevant modules are processed.

## Project Structure

The project is organized as a Cargo workspace with three member crates:

1. **rustantic:**  
   Contains core functionality including:
   - Definitions of metadata structs.
   - A collector and generator for the Python code.

2. **rustantic-macros:**  
   Contains procedural macros (e.g., `#[my_pydantic]`) that:
   - Automatically inject the `#[pyclass]` attribute.
   - Optionally register type metadata (if using runtime collection).

3. **rustantic_test:**  
   A sample/test crate that:
   - Uses the macros from `rustantic-macros`.
   - Contains example Rust structs (annotated with `#[pydantic]` and optionally `#[new]`).
   - Has a `build.rs` script that scans the source code and generates a Python files in (`rustantic_test/generated`) with the corresponding Pydantic models.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Maturin](https://maturin.rs/) (if you plan to build a Python wheel)
- Python 3.x

### Create required files
1. **```build.rs```:**  
```rust
// build.rs
use std::env;
use rustantic::generator::PydanticCodeGenerator;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let pydantic_module = "generated";
    let module_name = "rustantic_test";
    let py_path = format!("{}/{}/{}", manifest_dir, module_name, pydantic_module);
    let models_package_name = format!("{}.{}", module_name, pydantic_module);
    PydanticCodeGenerator::generate(module_name, &py_path, &manifest_dir, &models_package_name);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
```
2. **```src/lib.rs```:**:
```rs
//src/lib.rs
use pyo3::prelude::*;
use rustantic_macros::pydantic;

#[pydantic]
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
```
### Generated files:
```python
# my_class.py
# Generated by rustantic version: 0.2.0
from pydantic import BaseModel
import rustantic_test
from pydantic import Field
from rustantic_test.generated.nested import Nested as PydanticNested

class MyClass(BaseModel):
    name: str
    num2: int = Field(..., ge=0)
    nested: PydanticNested

    def to_rs(self):
        return rustantic_test.MyClass(
            name=self.name,
            num2=self.num2,
            nested=self.nested.to_rs(),
        )

# nested.py
# Generated by rustantic version: 0.2.0
from pydantic import BaseModel
import rustantic_test
from pydantic import Field

class Nested(BaseModel):
    name: str
    num: int = Field(..., ge=0)

    def to_rs(self):
        return rustantic_test.Nested(
            name=self.name,
            num=self.num,
        )
```
