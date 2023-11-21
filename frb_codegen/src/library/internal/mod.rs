use crate::library::commands::cargo_metadata::execute_cargo_metadata;
use crate::library::commands::cbindgen::{cbindgen_raw, default_cbindgen_config};
use crate::library::commands::ffigen::{
    ffigen_raw, FfigenCommandConfig, FfigenCommandConfigHeaders,
};
use crate::utils::path_utils::path_to_string;
use convert_case::{Case, Casing};
use log::info;
use serde_json::json;
use std::env;
use std::path::{Path, PathBuf};

pub fn generate() -> anyhow::Result<()> {
    let repo_base_dir = compute_repo_base_dir()?;
    info!("Determine repo_base_dir={repo_base_dir:?}");

    generate_frb_rust_cbindgen(&repo_base_dir)?;
    generate_allo_isolate_cbindgen(&repo_base_dir)?;

    ffigen(&repo_base_dir)?;

    Ok(())
}

fn compute_repo_base_dir() -> anyhow::Result<PathBuf> {
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .parent()
        .unwrap()
        .to_owned())
}

fn generate_frb_rust_cbindgen(repo_base_dir: &PathBuf) -> anyhow::Result<()> {
    info!("generate_frb_rust_cbindgen");
    cbindgen(repo_base_dir, &repo_base_dir.join("frb_rust"), "frb_rust")
}

fn generate_allo_isolate_cbindgen(repo_base_dir: &PathBuf) -> anyhow::Result<()> {
    info!("generate_allo_isolate_cbindgen");

    let metadata = execute_cargo_metadata(&repo_base_dir.join("frb_codegen/Cargo.toml"))?;

    let package_name = "allo-isolate";
    let package = (metadata.packages.iter())
        .filter(|package| package.name == package_name)
        .next()
        .unwrap();
    let rust_crate_dir = package.manifest_path.as_std_path().parent().unwrap();

    cbindgen(repo_base_dir, rust_crate_dir, "allo_isolate")
}

fn cbindgen(repo_base_dir: &PathBuf, rust_crate_dir: &Path, name: &str) -> anyhow::Result<()> {
    let c_path = repo_base_dir.join(format!(
        "frb_dart/lib/src/ffigen_generated/{}.h",
        name.to_case(Case::Snake)
    ));
    cbindgen_raw(default_cbindgen_config(), rust_crate_dir, &c_path)
}

fn ffigen(repo_base_dir: &Path) -> anyhow::Result<()> {
    let headers = vec![
        repo_base_dir.join("frb_rust/src/dart_api/dart_native_api.h"),
        repo_base_dir.join("frb_dart/lib/src/ffigen_generated/frb_rust.h"),
        repo_base_dir.join("frb_dart/lib/src/ffigen_generated/allo_isolate.h"),
    ];

    ffigen_raw(
        &FfigenCommandConfig {
            output: repo_base_dir.join("frb_dart/lib/src/ffigen_generated/multi_package.dart"),
            name: format!("MultiPackageCBinding"),
            headers: FfigenCommandConfigHeaders {
                entry_points: headers.clone(),
                include_directives: headers.clone(),
            },
            preamble: FFIGEN_PREAMBLE.to_owned(),
            description: FFIGEN_DESCRIPTION.to_owned(),
            ..Default::default()
        },
        &repo_base_dir.join("frb_dart"),
    )
}

const FFIGEN_PREAMBLE: &str = "// AUTO-GENERATED BY frb_codegen::internal command\n// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names, unused_field, library_private_types_in_public_api\n";
const FFIGEN_DESCRIPTION: &str = "generated by frb_codegen::internal command";
