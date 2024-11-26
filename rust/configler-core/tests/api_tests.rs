use configler_core;

#[test]
fn verify_lazy_builder_and_config_visibility() {
    let builder_result = configler_core::ConfigBuilder::new()
        .add_source(configler_core::SourceName::Environment)
        .add_source(configler_core::SourceName::YamlFile)
        .set_config_directory("./test_configs")
        .build();

    assert!(builder_result.is_ok());
}

// TODO verify custom source config visibility
// TODO verify new custom config sources can be added
// TODO verify pattern with sub config sources is possible