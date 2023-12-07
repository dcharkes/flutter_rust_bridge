use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::misc::{
    generate_class_from_fields, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::general_list::{
    general_list_impl_decode_body, generate_list_generate_allocate_func,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for DelegateWireRustTransferCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<String> {
        match &self.ir {
            ty @ IrTypeDelegate::StringList => Some(generate_class_from_fields(
                self.ir.clone(),
                self.context,
                &[
                    format!(
                        "ptr: *mut *mut {}",
                        WireRustTransferCstGenerator::new(ty.get_delegate(), self.context)
                            .rust_wire_type(Target::Io)
                    ),
                    "len: i32".to_owned(),
                ],
            )),
            _ => None,
        }
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        match &self.ir {
            IrTypeDelegate::Array(array) => {
                let acc =
                    Some(
                        format!(
                            "let vec: Vec<{}> = self.cst_decode(); flutter_rust_bridge::for_generated::from_vec_to_array(vec)",
                            array.inner().rust_api_type()
                        ),
                    );
                if is_js_value(&array.inner()) {
                    return Acc {
                        io: acc,
                        ..Default::default()
                    };
                }
                Acc::distribute(acc)
            },
            IrTypeDelegate::String => {
                Acc {
                    wasm: Some("self".into()),
                    io: Some("let vec: Vec<u8> = self.cst_decode(); String::from_utf8_lossy(&vec).into_owned()".into()),
                    ..Default::default()
                }
            },
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                Acc::distribute(Some("flutter_rust_bridge::ZeroCopyBuffer(self.cst_decode())".into()))
            },
            IrTypeDelegate::StringList => general_list_impl_decode_body(),
            IrTypeDelegate::PrimitiveEnum (IrTypeDelegatePrimitiveEnum{ ir, .. }) => {
                let enu = ir.get(self.context.ir_pack);
                let variants =
                    (enu.variants().iter().enumerate())
                        .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name.rust_style(), variant.name))
                        .collect_vec()
                        .join("\n");
                format!(
                    "match self {{
                        {}
                        _ => unreachable!(\"Invalid variant for {}: {{}}\", self),
                    }}",
                    variants,
                    enu.name.name
                ).into()
            },
            IrTypeDelegate::Time(ir) => {
                if ir == &IrTypeDelegateTime::Duration {
                    return Acc {
                        io: Some("chrono::Duration::microseconds(self)".into()),
                        wasm: Some("chrono::Duration::milliseconds(self)".into()),
                        ..Default::default()
                    };
                }
                let codegen_timestamp = "let flutter_rust_bridge::for_generated::Timestamp { s, ns } = flutter_rust_bridge::for_generated::cst_decode_timestamp(self);";
                let codegen_naive =
                    "chrono::NaiveDateTime::from_timestamp_opt(s, ns).expect(\"invalid or out-of-range datetime\")";
                let codegen_utc = format!("chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset({codegen_naive}, chrono::Utc)");
                let codegen_local = format!("chrono::DateTime::<chrono::Local>::from({codegen_utc})");
                let codegen_conversion = match ir {
                    IrTypeDelegateTime::Naive => codegen_naive,
                    IrTypeDelegateTime::Utc => codegen_utc.as_str(),
                    IrTypeDelegateTime::Local => codegen_local.as_str(),
                    IrTypeDelegateTime::Duration => unreachable!(),
                };
                Acc {
                    common: Some(format!("{codegen_timestamp}{codegen_conversion}")),
                    ..Default::default()
                }
            },
            IrTypeDelegate::TimeList(_) => {
                Acc::distribute(
                    Some(
                        "let vec: Vec<i64> = self.cst_decode(); vec.into_iter().map(CstDecoder::cst_decode).collect()".into()
                    )
                )
            }
            IrTypeDelegate::Uuid => Acc::distribute(
                Some(
                    "let single: Vec<u8> = self.cst_decode(); flutter_rust_bridge::for_generated::cst_decode_uuid_ref(single.as_slice())".into(),
                ),
            ),
            IrTypeDelegate::Uuids => Acc::distribute(
                Some(
                    "let multiple: Vec<u8> = self.cst_decode(); flutter_rust_bridge::for_generated::cst_decode_uuids(multiple)".into(),
                ),
            ),
            IrTypeDelegate::Backtrace | IrTypeDelegate::Anyhow => "self.cst_decode()".into(),
        }
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        Some(match &self.ir {
            IrTypeDelegate::String => {
                "self.as_string().expect(\"non-UTF-8 string, or not a string\")".into()
            }
            IrTypeDelegate::PrimitiveEnum (IrTypeDelegatePrimitiveEnum { repr, .. }) => format!(
                "(self.unchecked_into_f64() as {}).cst_decode()",
                WireRustTransferCstGenerator::new(repr.clone(), self.context).rust_wire_type(Target::Wasm)
            )
                .into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "flutter_rust_bridge::ZeroCopyBuffer(self.cst_decode())".into()
            }
            IrTypeDelegate::Time(_) => "CstDecoder::<i64>::cst_decode(self).cst_decode()".into(),
            IrTypeDelegate::TimeList(_) =>
                "self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::BigInt64Array>().to_vec().into_iter().map(CstDecoder::cst_decode).collect()".into(),
            IrTypeDelegate::Uuid | IrTypeDelegate::Uuids => {
                "self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>().to_vec().into_boxed_slice().cst_decode()"
                    .into()
            }
            IrTypeDelegate::Array(array) => format!(
                "let vec: Vec<{}> = self.cst_decode(); flutter_rust_bridge::for_generated::from_vec_to_array(vec)",
                array.inner().rust_api_type()
            )
                .into(),
            _ => return None,
        })
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        match &self.ir {
            list @ IrTypeDelegate::StringList => Acc {
                io: generate_list_generate_allocate_func(
                    &self.ir.safe_ident(),
                    &IrType::Delegate(list.clone()),
                    &list.get_delegate(),
                    self.context,
                )
                .into(),
                ..Default::default()
            },
            _ => Default::default(),
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_cst_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => JS_VALUE.into(),
            _ => WireRustTransferCstGenerator::new(self.ir.get_delegate(), self.context)
                .rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        WireRustTransferCstGenerator::new(self.ir.get_delegate(), self.context)
            .rust_wire_is_pointer(target)
    }
}