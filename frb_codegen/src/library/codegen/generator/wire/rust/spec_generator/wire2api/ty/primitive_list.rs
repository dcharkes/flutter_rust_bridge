use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::misc::{
    generate_class_from_fields, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorWire2apiTrait for PrimitiveListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
            &vec![
                format!(
                    "ptr: *mut {}",
                    WireRustGenerator::new(self.ir.primitive.clone(), self.context)
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: Some("self.into_vec()".into()),
            io: Some(
                "unsafe {
                    let wrap = support::box_from_leak_ptr(self);
                    support::vec_from_leak_ptr(wrap.ptr, wrap.len)
                }"
                .into(),
            ),
            ..Default::default()
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        match self.ir.primitive {
            IrTypePrimitive::Bool | IrTypePrimitive::Unit => Some("todo!()".into()),
            IrTypePrimitive::I64 | IrTypePrimitive::U64 => Some(
                format!(
                    "let buf = self.dyn_into::<{}>().unwrap();
                    let buf = js_sys::Uint8Array::new(&buf.buffer());
                    support::slice_from_byte_buffer(buf.to_vec()).into()",
                    rust_wasm_wire_type(&self.ir)
                )
                .into(),
            ),
            _ => Some(
                format!(
                    "self.unchecked_into::<{}>().to_vec().into()",
                    rust_wasm_wire_type(&self.ir)
                )
                .into(),
            ),
        }
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc {
            io: ExternFunc {
                func_name: format!("new_{}", self.ir.safe_ident()),
                params: vec![ExternFuncParam {
                    name: "len".to_owned(),
                    rust_type: "i32".to_owned(),
                    dart_type: Some("int".to_owned()),
                }],
                return_type: Some(format!(
                    "{}{}",
                    self.rust_wire_modifier(Target::Io),
                    self.rust_wire_type(Target::Io)
                )),
                body: format!(
                    "let ans = {} {{ ptr: support::new_leak_vec_ptr(Default::default(), len), len }};
                    support::new_leak_box_ptr(ans)",
                    self.rust_wire_type(Target::Io),
                ),
                target: Target::Io,
            }.into(),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            match self.ir.primitive {
                IrTypePrimitive::Bool | IrTypePrimitive::Unit => JS_VALUE.into(),
                _ => format!("Box<[{}]>", self.ir.primitive.rust_api_type()),
            }
        } else {
            format!("wire_{}", self.ir.safe_ident())
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
    }
}

fn rust_wasm_wire_type(ir: &IrTypePrimitiveList) -> &str {
    match &ir.primitive {
        IrTypePrimitive::U8 => "js_sys::Uint8Array",
        IrTypePrimitive::I8 => "js_sys::Int8Array",
        IrTypePrimitive::U16 => "js_sys::Uint16Array",
        IrTypePrimitive::I16 => "js_sys::Int16Array",
        IrTypePrimitive::U32 | IrTypePrimitive::Usize => "js_sys::Uint32Array",
        IrTypePrimitive::I32 | IrTypePrimitive::Isize => "js_sys::Int32Array",
        IrTypePrimitive::U64 => "js_sys::BigUint64Array",
        IrTypePrimitive::I64 => "js_sys::BigInt64Array",
        IrTypePrimitive::F32 => "js_sys::Float32Array",
        IrTypePrimitive::F64 => "js_sys::Float64Array",
        IrTypePrimitive::Bool | IrTypePrimitive::Unit => "js_sys::Array",
    }
}