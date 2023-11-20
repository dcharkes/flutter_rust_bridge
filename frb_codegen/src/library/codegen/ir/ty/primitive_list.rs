use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypePrimitiveList {
    pub primitive: IrTypePrimitive,
}
}

impl IrTypeTrait for IrTypePrimitiveList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        IrType::Primitive(self.primitive.clone()).visit_types(f, ir_pack);
    }

    fn safe_ident(&self) -> String {
        format!("list_prim_{}", self.primitive.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }
}