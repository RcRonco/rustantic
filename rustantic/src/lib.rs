pub mod collector;
pub mod generators;
pub mod models;

pub fn generate(package_name: &str, base_path: &str, lib_path: &str, models_package_name: &str) {
    let mut generator = generators::generator::PydanticGenerator::new(
        package_name.to_string(),
        base_path.to_string(),
        models_package_name.to_string(),
        collector::MetadataCollector::new(lib_path),
    )
    .register::<generators::StructCodeGenerator>()
    .register::<generators::EnumCodeGenerator>()
    .register::<generators::UnionCodeGenerator>();

    generator.generate();
}
