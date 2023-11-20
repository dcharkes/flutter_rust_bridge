use crate::enum_map;
use anyhow::bail;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum_macros::EnumIter)]
pub enum Target {
    Io,
    Wasm,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum_macros::EnumIter)]
pub enum TargetOrCommon {
    Common,
    Io,
    Wasm,
}

enum_map!(
    TargetOrCommonMap, TargetOrCommon;
    Common, Io, Wasm;
    common, io, wasm;
);

impl TryFrom<TargetOrCommon> for Target {
    type Error = anyhow::Error;

    fn try_from(src: TargetOrCommon) -> Result<Self, Self::Error> {
        Ok(match src {
            TargetOrCommon::Common => bail!("Cannot convert TargetOrCommon::Common to Target"),
            TargetOrCommon::Io => Target::Io,
            TargetOrCommon::Wasm => Target::Wasm,
        })
    }
}

impl From<Target> for TargetOrCommon {
    fn from(value: Target) -> Self {
        match value {
            Target::Io => TargetOrCommon::Io,
            Target::Wasm => TargetOrCommon::Wasm,
        }
    }
}

impl TargetOrCommon {
    pub(crate) fn to_target_or(&self, when_common: Target) -> Target {
        match self {
            TargetOrCommon::Common => when_common,
            TargetOrCommon::Io | TargetOrCommon::Wasm => (*self).try_into().unwrap(),
        }
    }
}