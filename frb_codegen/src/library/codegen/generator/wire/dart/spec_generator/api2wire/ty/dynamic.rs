use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::*;

impl<'a> WireDartGeneratorApi2wireTrait for DynamicWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc::default()
    }

    fn dart_wire_type(&self, _target: Target) -> String {
        panic!("Functions cannot receive dynamic parameters.")
    }
}