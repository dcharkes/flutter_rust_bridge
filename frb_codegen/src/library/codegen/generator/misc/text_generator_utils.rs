use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{TargetOrCommon, TargetOrCommonMap};
use crate::utils::file_utils::create_dir_all_and_write;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub(crate) fn write_code_for_targets(
    text: &Acc<Option<String>>,
    output_path: &TargetOrCommonMap<PathBuf>,
) -> anyhow::Result<()> {
    for target in TargetOrCommon::iter() {
        if let Some(text) = &text[target] {
            let path = &output_path[target];
            create_dir_all_and_write(path, text)?;
        }
    }
    Ok(())
}

pub(crate) fn section_header_comment(section_name: &str) -> String {
    format!("// Section: {section_name}\n")
}

pub(crate) fn generate_text_respecting_wasm_flag(
    raw: Acc<String>,
    wasm_enabled: bool,
) -> Acc<Option<String>> {
    raw.map(|value, target| (target != TargetOrCommon::Wasm || wasm_enabled).then(|| value))
}