pub(crate) mod internal_config;
mod spec_generator;
mod text_generator;

use crate::codegen::generator::misc::{PathText, PathTexts};
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;

pub(crate) struct GeneratorWireCOutput {
    pub output_texts: PathTexts,
    pub c_file_content: String,
}

pub(crate) fn generate(
    config: &GeneratorWireCInternalConfig,
    extern_func_names: Vec<String>,
    extern_struct_names: Vec<String>,
    rust_output_texts: &PathTexts,
) -> anyhow::Result<GeneratorWireCOutput> {
    let spec = spec_generator::generate(
        config,
        extern_func_names,
        extern_struct_names,
        rust_output_texts,
    )?;
    let text = text_generator::generate(spec)?;
    Ok(GeneratorWireCOutput {
        output_texts: PathTexts(vec![PathText::new(
            config.c_output_path.clone(),
            text.clone(),
        )]),
        c_file_content: text,
    })
}