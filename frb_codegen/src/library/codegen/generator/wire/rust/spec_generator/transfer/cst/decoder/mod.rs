use crate::library::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;
use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::{
    WireRustTransferCstGenerator, WireRustTransferCstGeneratorContext,
};
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use serde::Serialize;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::impl_decode_trait::generate_impl_decode;

mod impl_decode_trait;
mod impl_new_with_nullptr;
mod misc;
pub(crate) mod ty;

#[derive(Serialize)]
pub(crate) struct WireDartOutputSpecTransferCstDecoder {
    pub allocate_funcs: Acc<Vec<WireRustOutputCode>>,
    pub impl_decode: Acc<Vec<WireRustOutputCode>>,
    pub decoder_class: Acc<Vec<WireRustOutputCode>>,
    pub impl_new_with_nullptr: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustTransferCstGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecTransferCstDecoder {
    WireDartOutputSpecTransferCstDecoder {
        allocate_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| {
                WireRustTransferCstGenerator::new(ty.clone(), context).generate_allocate_funcs()
            })
            .collect(),
        impl_decode: generate_impl_decode(&cache.distinct_input_types, context),
        decoder_class: Acc::new_io(
            cache
                .distinct_input_types
                .iter()
                .filter_map(|ty| {
                    WireRustTransferCstGenerator::new(ty.clone(), context).generate_decoder_class()
                })
                .map(|x| x.into())
                .collect(),
        ),
        impl_new_with_nullptr: Acc::new_io(generate_impl_new_with_nullptr(
            &cache.distinct_input_types,
            context,
        )),
    }
}