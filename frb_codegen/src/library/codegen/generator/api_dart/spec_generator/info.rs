use crate::codegen::generator::api_dart::spec_generator::base::*;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use convert_case::{Case, Casing};
use enum_dispatch::enum_dispatch;
use itertools::Itertools;

#[enum_dispatch]
pub(crate) trait ApiDartGeneratorInfoTrait {
    fn dart_api_type(&self) -> String;
}

impl<'a> ApiDartGeneratorInfoTrait for BoxedApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        inner.dart_api_type()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DartOpaqueApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        "Object".to_owned()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DelegateApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir {
            IrTypeDelegate::Array(array) => array.dart_api_type(self.context),
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                ApiDartGenerator::new(self.ir.get_delegate(), self.context).dart_api_type()
            }
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                ApiDartGenerator::new(IrType::EnumRef(ir.clone()), self.context).dart_api_type()
            }
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeDelegateTime::Local | IrTypeDelegateTime::Utc | IrTypeDelegateTime::Naive => {
                    "DateTime".to_string()
                }
                IrTypeDelegateTime::Duration => "Duration".to_string(),
            },
            IrTypeDelegate::TimeList(
                IrTypeDelegateTime::Local | IrTypeDelegateTime::Utc | IrTypeDelegateTime::Naive,
            ) => "List<DateTime>".to_string(),
            IrTypeDelegate::TimeList(IrTypeDelegateTime::Duration) => "List<Duration>".to_string(),
            IrTypeDelegate::Uuid => "UuidValue".to_owned(),
            IrTypeDelegate::Uuids => "List<UuidValue>".to_owned(),
            IrTypeDelegate::Backtrace => "String".to_string(),
            IrTypeDelegate::Anyhow => "FrbAnyhowException".to_string(),
        }
    }
}

impl IrTypeDelegateArray {
    pub(crate) fn dart_api_type(&self, context: ApiDartGeneratorContext) -> String {
        match self {
            IrTypeDelegateArray::GeneralArray { general, length } => {
                format!(
                    "{}Array{length}",
                    ApiDartGenerator::new(general.clone(), context).dart_api_type()
                )
            }
            IrTypeDelegateArray::PrimitiveArray { primitive, length } => {
                format!(
                    "{}Array{length}",
                    primitive.safe_ident().to_case(Case::Pascal)
                )
            }
        }
    }
}

impl<'a> ApiDartGeneratorInfoTrait for DynamicApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        "dynamic".to_owned()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for EnumRefApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.ident.0.to_string()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for GeneralListApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        format!("List<{}>", inner.dart_api_type())
    }
}

impl<'a> ApiDartGeneratorInfoTrait for OptionalApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        format!("{}?", inner.dart_api_type())
    }
}

impl<'a> ApiDartGeneratorInfoTrait for OptionalListApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        format!("List<{}?>", inner.dart_api_type())
    }
}

impl<'a> ApiDartGeneratorInfoTrait for PrimitiveApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir {
            IrTypePrimitive::U8
            | IrTypePrimitive::I8
            | IrTypePrimitive::U16
            | IrTypePrimitive::I16
            | IrTypePrimitive::U32
            | IrTypePrimitive::I32
            | IrTypePrimitive::Usize
            | IrTypePrimitive::Isize => "int",
            IrTypePrimitive::U64 | IrTypePrimitive::I64 => "BigInt",
            IrTypePrimitive::F32 | IrTypePrimitive::F64 => "double",
            IrTypePrimitive::Bool => "bool",
            IrTypePrimitive::Unit => "void",
        }
        .to_owned()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for PrimitiveListApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        match &self.ir.primitive {
            IrTypePrimitive::U8 => "Uint8List",
            IrTypePrimitive::I8 => "Int8List",
            IrTypePrimitive::U16 => "Uint16List",
            IrTypePrimitive::I16 => "Int16List",
            IrTypePrimitive::U32 => "Uint32List",
            IrTypePrimitive::I32 => "Int32List",
            IrTypePrimitive::U64 => "Uint64List",
            IrTypePrimitive::I64 => "Int64List",
            IrTypePrimitive::F32 => "Float32List",
            IrTypePrimitive::F64 => "Float64List",
            _ => panic!("does not support {:?} yet", &self.ir.primitive),
        }
        .to_string()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for RecordApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let values = self
            .ir
            .values
            .iter()
            .map(|ty| ApiDartGenerator::new(ty.clone(), self.context).dart_api_type())
            .collect_vec()
            .join(",");
        if self.ir.values.len() == 1 {
            format!("({values},)")
        } else {
            format!("({values})")
        }
    }
}

impl<'a> ApiDartGeneratorInfoTrait for RustOpaqueApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        let inner = ApiDartGenerator::new(self.ir.inner.clone(), self.context);
        inner.dart_api_type()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for StructRefApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        self.ir.ident.0.to_string()
    }
}

impl<'a> ApiDartGeneratorInfoTrait for UnencodableApiDartGenerator<'a> {
    fn dart_api_type(&self) -> String {
        unreachable!()
    }
}