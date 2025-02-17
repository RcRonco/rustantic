pub mod enum_generator;
pub mod field_generator;
pub mod generator;
pub mod generator_base;
pub mod struct_generator;
pub mod union_generator;

pub(crate) use enum_generator::EnumCodeGenerator;
pub(crate) use struct_generator::StructCodeGenerator;
pub(crate) use union_generator::UnionCodeGenerator;

pub fn generate(package_name: &str, base_path: &str, lib_path: &str, models_package_name: &str) {
    use crate::collector;

    let mut generator = generator::PydanticGenerator::new(
        package_name.to_string(),
        base_path.to_string(),
        models_package_name.to_string(),
        collector::MetadataCollector::new(lib_path),
    )
    .register::<StructCodeGenerator>()
    .register::<EnumCodeGenerator>()
    .register::<UnionCodeGenerator>();

    generator.generate();
}
