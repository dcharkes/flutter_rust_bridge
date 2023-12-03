use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpecRust2Dart {
    pub(crate) impl_wire2api: Acc<Vec<WireDartOutputCode>>,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecRust2Dart {
    WireDartOutputSpecRust2Dart {
        impl_wire2api: cache
            .distinct_output_types
            .iter()
            .map(|ty| Acc::new_common(generate_impl_wire2api(ty, context)))
            .collect(),
    }
}

fn generate_impl_wire2api(ty: &IrType, context: WireDartGeneratorContext) -> WireDartOutputCode {
    let body = WireDartGenerator::new(ty.clone(), context).generate_impl_wire2api_body();
    let api_impl_body = format!(
        "{dart_api_type} _wire2api_{safe_ident}(dynamic raw) {{
            {body}
        }}
        ",
        dart_api_type =
            ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
        safe_ident = ty.safe_ident(),
    );
    WireDartOutputCode {
        api_impl_body,
        ..Default::default()
    }
}