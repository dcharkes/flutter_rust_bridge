use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;

impl<'a> WireRustGeneratorMiscTrait for DelegateWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        if let IrTypeDelegate::PrimitiveEnum(enu) = &self.ir {
            WireRustGenerator::new(enu.ir.clone(), self.context).wrapper_struct_name()
        } else {
            None
        }

        // TODO https://github.com/fzyzcjy/yplusplus/issues/11145#issuecomment-1816273032
        // forward_delegate_primitive_enum!(self, wrapper_struct_name(), None)
    }

    // TODO old code has this, but I think do not need?
    // fn generate_static_checks(&self) -> Option<String> {
    //     forward_delegate_primitive_enum!(self, static_checks(), None)
    // }

    // TODO rm this, since we will visit all sub-types to generate
    // fn generate_imports(&self) -> Option<Vec<String>> {
    //     forward_delegate_primitive_enum!(self, imports(), None)
    // }
}