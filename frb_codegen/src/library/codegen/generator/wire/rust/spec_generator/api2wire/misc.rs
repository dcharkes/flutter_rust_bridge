use crate::codegen::ir::namespace::NamespacedName;

pub(crate) fn generate_impl_into_into_dart(
    name: &NamespacedName,
    wrapper_name: &Option<NamespacedName>,
) -> String {
    let body = if let Some(wrapper_name) = wrapper_name {
        wrapper_name.name.clone()
    } else {
        "self".to_owned()
    };

    let wrapper_name = (wrapper_name.as_ref().map(|x| x.name.clone())).unwrap_or(name.name.clone());
    let name = &name.name;

    format!(
        "impl flutter_rust_bridge::rust2dart::IntoIntoDart<{wrapper_name}> for {name} {{
            fn into_into_dart(self) -> {wrapper_name} {{
                {body}
            }}
        }}"
    )
}
