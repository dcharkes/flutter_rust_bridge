use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::enumeration::IrTypeEnumRef;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::optional_list::IrTypeOptionalList;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::record::IrTypeRecord;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use crate::codegen_generator_structs;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use enum_dispatch::enum_dispatch;
use paste::paste;

#[enum_dispatch(WireRustGeneratorImplTrait)]
#[enum_dispatch(WireRustGeneratorApi2wireTrait)]
#[enum_dispatch(WireRustGeneratorWire2apiTrait)]
#[enum_dispatch(WireRustGeneratorMiscTrait)]
pub(crate) enum WireRustGenerator<'a> {
    Boxed(BoxedWireRustGenerator<'a>),
    DartOpaque(DartOpaqueWireRustGenerator<'a>),
    Delegate(DelegateWireRustGenerator<'a>),
    Dynamic(DynamicWireRustGenerator<'a>),
    EnumRef(EnumRefWireRustGenerator<'a>),
    GeneralList(GeneralListWireRustGenerator<'a>),
    Optional(OptionalWireRustGenerator<'a>),
    OptionalList(OptionalListWireRustGenerator<'a>),
    Primitive(PrimitiveWireRustGenerator<'a>),
    PrimitiveList(PrimitiveListWireRustGenerator<'a>),
    Record(RecordWireRustGenerator<'a>),
    RustOpaque(RustOpaqueWireRustGenerator<'a>),
    StructRef(StructRefWireRustGenerator<'a>),
    Unencodable(UnencodableWireRustGenerator<'a>),
}

codegen_generator_structs!(
    WireRustGenerator;

    Boxed,
    DartOpaque,
    Delegate,
    Dynamic,
    EnumRef,
    GeneralList,
    Optional,
    OptionalList,
    Primitive,
    PrimitiveList,
    Record,
    RustOpaque,
    StructRef,
    Unencodable,
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireRustGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireRustInternalConfig,
    pub(crate) wire_dart_config: &'a GeneratorWireDartInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireRustGeneratorContext<'_> {
    pub(crate) fn as_wire_dart_context(&self) -> WireDartGeneratorContext {
        WireDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.wire_dart_config,
            wire_rust_config: self.config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.api_dart_config,
        }
    }
}