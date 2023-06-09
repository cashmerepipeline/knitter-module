

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
        .extern_path(".cashmere", "::manage_define::cashmere")
        .build_client(false)
        .build_server(false)
        .type_attribute("Reference", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_well_known_types(true)
        .compile(
            &["protocols/knitter_module.proto"],
            &["protocols", "../cashmere_core/protocols"],
        )?;

    manage_define::utils::generate_manage_defines(
        &vec!["manage_defines"],
        "src/ids_codes",
        Some("dart_packges/knitter_id_codes/lib/src"),
    );

    Ok(())
}
