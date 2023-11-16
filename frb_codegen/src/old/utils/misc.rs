use std::collections::HashSet;

use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::path::Path;

use crate::commands::cargo_expand;
use crate::utils::consts::DART_KEYWORDS;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use pathdiff::diff_paths;

pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
}

pub fn with_changed_file<F: FnOnce() -> crate::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> crate::Result<()> {
    let content_original = fs::read_to_string(path)?;
    fs::write(path, content_original.clone() + append_content)?;

    f()?;

    Ok(fs::write(path, content_original)?)
}

pub fn find_all_duplicates<T>(iter: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    iter.iter()
        .filter(|x| !uniq.insert(*x))
        .cloned()
        .collect::<Vec<_>>()
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name),
/// including those generated implicitly by frb
pub fn get_symbols_if_no_duplicates(configs: &[crate::Opts]) -> Result<Vec<String>, anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for config in configs {
        let raw_ir_pack = config.get_ir_pack()?;

        // for checking explicit api duplication
        explicit_raw_symbols.extend(raw_ir_pack.funcs.iter().map(|f| f.name.clone()));

        // for avoiding redundant generation in dart
        all_symbols.extend(raw_ir_pack.get_all_symbols(config));
    }
    let duplicates = find_all_duplicates(&explicit_raw_symbols);
    if !duplicates.is_empty() {
        let duplicated_symbols = duplicates.join(",");

        let (symbol_str, verb_str) = if duplicates.len() == 1 {
            ("symbol", "has")
        } else {
            ("symbols", "have")
        };
        return Err(anyhow!(
            "{} [{}] {} already been defined",
            symbol_str,
            duplicated_symbols,
            verb_str
        ));
    }

    check_for_keywords(&all_symbols)?;

    Ok(all_symbols)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, serde::Serialize)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait PathExt {
    fn file_name_str(&self) -> Option<&str>;
    fn directory_name_str(&self) -> Option<&str>;

    fn get_relative_path_to<P>(&self, path: P, exclude_file: bool) -> String
    where
        P: AsRef<Path>;
}

impl PathExt for std::path::Path {
    #[inline]
    fn file_name_str(&self) -> Option<&str> {
        self.file_name().and_then(std::ffi::OsStr::to_str)
    }
    #[inline]
    fn directory_name_str(&self) -> Option<&str> {
        self.parent().and_then(|p| p.to_str())
    }
    #[inline]
    fn get_relative_path_to<P>(&self, path: P, exclude_file: bool) -> String
    where
        P: AsRef<Path>,
    {
        if exclude_file {
            let src = self.parent().and_then(|p| p.to_str()).unwrap();
            diff_paths(path, src).unwrap().to_str().unwrap().to_owned()
        } else {
            diff_paths(path, self).unwrap().to_str().unwrap().to_owned()
        }
    }
}

/// For structs that only has an `inner` serializable property that
/// would be better (de)serialized as a newtype.
#[macro_export]
macro_rules! derive_serde_inner_as_newtype {
    ($($type:ident),*) => {$(
        impl ::serde::Serialize for $type {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                s.serialize_newtype_struct(::std::stringify!($type), self.inner.as_ref())
            }
        }
    )*};
}
