use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::enumeration::parse_wrapper_name_into_dart_name_and_self_path;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::forward_delegate_primitive_enum;
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for DelegateWireRustGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        if let IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) = &self.ir {
            let src = ir.get(self.context.ir_pack);
            let (name, self_path) =
                parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

            let self_ref = self.generate_access_object_core("self".to_owned());
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("{}::{} => {},", self_path, variant.name, idx))
                .collect_vec()
                .join("\n");
            let into_into_dart = generate_impl_into_into_dart(&src.name, &src.wrapper_name);
            return Some(format!(
                "impl flutter_rust_bridge::IntoDart for {name} {{
                    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {{
                        match {self_ref} {{
                            {variants}
                        }}.into_dart()
                    }}
                }}
                impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for {name} {{}}
                {into_into_dart}
                "
            ));
        }
        None
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        forward_delegate_primitive_enum!(self, generate_access_object_core(obj), obj)
    }
}