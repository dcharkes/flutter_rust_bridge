use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{TargetOrCommon, TargetOrCommonMap};
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::enum_map;
use crate::utils::file_utils::create_dir_all_and_write;
use anyhow::bail;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::Add;
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub(crate) mod structs_macro;
pub(crate) mod target;
pub(crate) mod text_generator_utils;

/// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap and
/// therefore do not implement [Send]. More specifically, these are types wasm-bindgen
/// can't handle yet.
pub fn is_js_value(ty: &IrType) -> bool {
    match ty {
        IrType::GeneralList(_)
        | IrType::OptionalList(_)
        | IrType::StructRef(_)
        | IrType::EnumRef(_)
        | IrType::RustOpaque(_)
        | IrType::DartOpaque(_)
        | IrType::Record(_) => true,
        IrType::Boxed(IrTypeBoxed { inner, .. }) => is_js_value(inner),
        IrType::Delegate(inner) => is_js_value(&inner.get_delegate()),
        IrType::Optional(inner) => is_js_value(&inner.inner),
        IrType::Primitive(_) | IrType::PrimitiveList(_) => false,
        IrType::Dynamic(_) | IrType::Unencodable(_) => unreachable!(),
    }
}

#[derive(Clone)]
pub(crate) struct PathText {
    pub path: PathBuf,
    pub text: String,
}

impl PathText {
    pub(crate) fn new(path: PathBuf, text: String) -> Self {
        Self { path, text }
    }
}

#[derive(Clone)]
pub(crate) struct PathTexts(pub Vec<PathText>);

impl Add for PathTexts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0, rhs.0].concat())
    }
}

impl PathTexts {
    pub(crate) fn new_from_targets(
        path: &TargetOrCommonMap<PathBuf>,
        text: &Acc<Option<String>>,
    ) -> Self {
        Self(
            TargetOrCommon::iter()
                .filter_map(|target| {
                    text[target]
                        .clone()
                        .map(|text_for_target| PathText::new(path[target].clone(), text_for_target))
                })
                .collect_vec(),
        )
    }

    pub(crate) fn write_to_disk(&self) -> anyhow::Result<()> {
        for item in self.0.iter() {
            create_dir_all_and_write(&item.path, &item.text)?;
        }
        Ok(())
    }

    pub(crate) fn paths(&self) -> Vec<PathBuf> {
        self.0.iter().map(|item| item.path.clone()).collect_vec()
    }
}