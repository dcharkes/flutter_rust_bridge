use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartGeneratorApi2wireTrait for PrimitiveWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        match self.ir {
            IrTypePrimitive::I64 | IrTypePrimitive::U64 => Acc {
                io: Some("return raw;".into()),
                wasm: Some("return castNativeBigInt(raw);".into()),
                ..Default::default()
            },
            _ => "return raw;".into(),
        }
    }
}
