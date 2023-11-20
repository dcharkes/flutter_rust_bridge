use crate::codegen::generator::api_dart::spec_generator::class::ty::structure_method::generate_api_method;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_metadata,
};
use crate::codegen::ir::func::{IrFuncOwnerInfo, IrFuncOwnerInfoMethod};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use itertools::Itertools;

impl<'a> ApiDartGeneratorClassTrait for StructRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let methods = self.context.ir_pack
            .funcs
            .iter()
            .filter(|f| {
                matches!(&f.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod{ struct_name, .. }) if struct_name == &src.name)
            })
            .map(|func| generate_api_method(func, src, self.context))
            .collect_vec();

        Some(if src.using_freezed() {
            self.generate_mode_freezed(src, &comments, &metadata, &methods)
        } else {
            self.generate_mode_non_freezed(src, &comments, &metadata, &methods)
        })
    }
}