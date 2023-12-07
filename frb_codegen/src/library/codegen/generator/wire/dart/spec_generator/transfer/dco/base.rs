use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(WireDartTransferDcoGeneratorImplTrait)]
    #[enum_dispatch(WireDartTransferDcoGeneratorEncoderTrait)]
    #[enum_dispatch(WireDartTransferDcoGeneratorDecoderTrait)]
    WireDartTransferDcoGenerator
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartTransferDcoGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireDartInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireDartTransferDcoGeneratorContext<'_> {
    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.api_dart_config,
        }
    }
}