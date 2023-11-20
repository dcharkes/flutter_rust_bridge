use super::*;

impl IrTypeTrait for IrTypeDynamic {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String {
        "dartabi".to_owned()
    }
}