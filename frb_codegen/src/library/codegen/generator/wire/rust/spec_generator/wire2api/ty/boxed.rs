use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::misc::target::TargetOrCommon::*;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGenerator;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorWire2apiTrait for BoxedWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let box_inner = self.ir.inner.as_ref();
        let exist_in_real_api = self.ir.exist_in_real_api;
        Acc::new(|target| match (target, self.ir.inner.as_ref()) {
            (Io, IrType::Primitive(_)) => Some(format!(
                "unsafe {{ {extra}support::box_from_leak_ptr(self) }}",
                extra = if exist_in_real_api { "" } else { "*" }
            )),
            (Io | Wasm, ir) if ir.is_array() => Some(format!(
                "Wire2Api::<{}>::wire2api(self).into()",
                box_inner.rust_api_type()
            )),
            (Io, _) => Some(format!(
                "let wrap = unsafe {{ support::box_from_leak_ptr(self) }};
                Wire2Api::<{}>::wire2api(*wrap).into()",
                box_inner.rust_api_type()
            )),
            _ => None,
        })
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        (self.ir.exist_in_real_api).then(|| match &*self.ir.inner {
            IrType::Delegate(IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum {
                repr,
                ..
            })) => format!(
                "let ptr: Box<{}> = self.wire2api(); Box::new(ptr.wire2api())",
                repr.rust_api_type()
            )
            .into(),
            IrType::Delegate(IrTypeDelegate::Array(array)) => format!(
                "let vec: Vec<{}> = self.wire2api(); Box::new(support::from_vec_to_array(vec))",
                array.inner().rust_api_type()
            )
            .into(),
            _ => "Box::new(self.wire2api())".into(),
        })
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        if self.ir.inner.is_array() {
            return Acc::default();
        }
        let func_name = format!("new_{}", self.ir.safe_ident());
        if self.ir.inner.is_primitive() {
            Acc {
                io: ExternFunc {
                    func_name,
                    params: vec![ExternFuncParam {
                        name: "value".to_owned(),
                        rust_type: WireRustGenerator::new(self.ir.inner.clone(), self.context)
                            .rust_wire_type(Target::Io),
                        dart_type: Some(
                            WireDartGenerator::new(
                                *self.ir.inner.clone(),
                                self.context.as_wire_dart_context(),
                            )
                            .dart_wire_type(Target::Io),
                        ),
                    }],
                    return_type: Some(format!(
                        "*mut {}",
                        WireRustGenerator::new(self.ir.inner.clone(), self.context)
                            .rust_wire_type(Target::Io)
                    )),
                    body: "support::new_leak_box_ptr(value)".to_owned(),
                    target: Target::Io,
                }
                .into(),
                ..Default::default()
            }
        } else {
            Acc {
                io: ExternFunc {
                    func_name,
                    params: vec![],
                    return_type: Some(
                        [
                            self.rust_wire_modifier(Target::Io),
                            self.rust_wire_type(Target::Io),
                        ]
                        .concat(),
                    ),
                    body: format!(
                        "support::new_leak_box_ptr({}::new_with_null_ptr())",
                        WireRustGenerator::new(self.ir.inner.clone(), self.context)
                            .rust_wire_type(Target::Io)
                    ),
                    target: Target::Io,
                }
                .into(),
                ..Default::default()
            }
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm && self.ir.inner.is_primitive() {
            JS_VALUE.into()
        } else {
            WireRustGenerator::new(self.ir.inner.clone(), self.context).rust_wire_type(target)
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        (target != Target::Wasm)
            || !is_js_value(&self.ir.inner)
                && !self.ir.inner.is_array()
                && !self.ir.inner.is_primitive()
    }
}