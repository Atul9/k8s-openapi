const MIN: usize = 8;
const MAX: usize = 16;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let enabled_version = {
        let mut enabled_versions = (MIN..=MAX).filter(|v| std::env::var(format!("CARGO_FEATURE_V1_{}", v)).is_ok());
        let v1 = enabled_versions.next().expect("\n\
            None of the v1_* features are enabled on the k8s-openapi crate.\n\
            \n\
            The k8s-openapi crate requires a feature to be enabled to indicate which version of Kubernetes it should support.\n\
            \n\
            If you're using k8s-openapi in a binary crate, enable the feature corresponding to the minimum version of API server that you want to support. \
            It may be possible that your binary crate does not directly depend on k8s-openapi. In this case, add a dependency on k8s-openapi, then enable \
            the corresponding feature.\n\
            \n\
            If you're using k8s-openapi in a library crate, add a dev-dependency on k8s-openapi and enable one of the features there. This way \
            the feature will be enabled when buildings tests and examples of your library, but not when building the library itself. \
            It may be possible that your library crate does not directly depend on k8s-openapi. In this case, add a dev-dependency on k8s-openapi, \
            then enable the corresponding feature.\n\
            \n\
            Library crates *must not* enable any features in their direct dependency on k8s-openapi, only in their dev-dependency. \
            The choice of Kubernetes version to support should be left to the final binary crate, so only the binary crate should enable a specific feature. \
            If library crates also enable features, it can cause multiple features to be enabled simultaneously, which k8s-openapi does not support.\n\
            \n\
            If you want to restrict your library crate to support only a single specific version or range of versions of Kubernetes, \
            please use the k8s_* version-specific macros to emit different code based on which feature gets enabled in the end.\
        ");
        if let Some(v2) = enabled_versions.next() {
            panic!(
                "\n\
                    Both v1_{} and v1_{} features are enabled on the k8s-openapi crate. Only one feature can be enabled at the same time.\n\
                    \n\
                    The feature indicates which version of Kubernetes the k8s-openapi crate should support.\n\
                    \n\
                    If you have enabled both of these features yourself, please remove one of them. If you are writing a library crate, \
                    do not enable any features at all. Library crates *must not* enable any features on the k8s-openapi crate.\n\
                    \n\
                    If you have not enabled one or both of these features yourself, then one of the library crates in your dependency graph *has*. \
                    Locate which library crates in your dependency graph depend on k8s-openapi and enable one of its features, and file a bug against them. \
                    You can search your Cargo.lock for \"k8s-openapi\" to discover these crates.\
                ",
                v1,
                v2,
            );
        }
        v1
    };

    let mut f = {
        let mut out_file: std::path::PathBuf = std::env::var_os("OUT_DIR").ok_or_else(|| "OUT_DIR not set")?.into();
        out_file.push("conditional_compilation_macros.rs");
        std::io::BufWriter::new(std::fs::File::create(out_file)?)
    };

    for v in MIN..=MAX {
        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` feature is enabled, otherwise it evaluates to nothing.", v)?;
        writeln!(f, "///")?;
        writeln!(f, "/// # Examples")?;
        writeln!(f, "///")?;
        writeln!(f, "/// ```rust")?;
        writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
        writeln!(f, "/// k8s_if_1_{}! {{", v)?;
        writeln!(f, "///     use k8s_openapi::api::core::v1 as api;")?;
        writeln!(f, "/// }}")?;
        writeln!(f, "/// ```")?;
        if enabled_version == v {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or higher feature is enabled, otherwise it evaluates to nothing.", v)?;
        if enabled_version >= v {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_ge_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        }
        writeln!(f)?;

        writeln!(f, "/// This macro evaluates to its contents if the `v1_{}` or lower feature is enabled, otherwise it evaluates to nothing.", v)?;
        if enabled_version <= v {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}", v)?;
        }
        else {
            writeln!(f, "#[macro_export] macro_rules! k8s_if_le_1_{} {{ ($($tt:tt)*) => {{ }}; }}", v)?;
        }
        writeln!(f)?;
    }

    writeln!(f, "/// A macro that emits a `match` expr with the given test expression and arms.")?;
    writeln!(f, "/// The match arms can be annotated with the other conditional compilation macros in this crate so that they're only emitted")?;
    writeln!(f, "/// if the predicate is true.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// # Examples")?;
    writeln!(f, "///")?;
    writeln!(f, "/// The `CustomResourceDefinition::create_custom_resource_definition` function returns an `HTTP 201 CREATED`")?;
    writeln!(f, "/// when it succeeds, but the codegen for v1.8 does not have a `Created` variant in the response type. So extracting the successful result from")?;
    writeln!(f, "/// the response requires matching `CreateCustomResourceDefinitionResponse::Other` for v1.8")?;
    writeln!(f, "/// and `CreateCustomResourceDefinitionResponse::Created` for v1.9 and above.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// Since `CreateCustomResourceDefinitionResponse::Created` does not exist in v1.8,")?;
    writeln!(f, "/// and `CreateCustomResourceDefinitionResponse::Other` would not be returned in v1.9 and above,")?;
    writeln!(f, "/// both arms need to be wrapped in conditional compilation predicates.")?;
    writeln!(f, "///")?;
    writeln!(f, "/// ```rust,no_run")?;
    writeln!(f, "/// # #[macro_use] extern crate k8s_openapi;")?;
    writeln!(f, "/// # use k8s_openapi::http;")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::{{")?;
    writeln!(f, "///     CreateCustomResourceDefinitionResponse,")?;
    writeln!(f, "///     CustomResourceDefinition,")?;
    writeln!(f, "/// }};")?;
    writeln!(f, "///")?;
    writeln!(f, "/// # fn main() -> Result<(), Box<std::error::Error>> {{")?;
    writeln!(f, "/// let response: CreateCustomResourceDefinitionResponse = unimplemented!();")?;
    writeln!(f, "/// let status_code: http::StatusCode = unimplemented!();")?;
    writeln!(f, "///")?;
    writeln!(f, "/// let custom_resource_definition: CustomResourceDefinition = k8s_match!((response, status_code), {{")?;
    writeln!(f, "///     k8s_if_1_8!((CreateCustomResourceDefinitionResponse::Other(value), http::StatusCode::CREATED) => {{")?;
    writeln!(f, "///         let value = match value? {{")?;
    writeln!(f, "///             Some(value) => value,")?;
    writeln!(f, "///             None => unimplemented!(), // Append more response data and try again.")?;
    writeln!(f, "///         }};")?;
    writeln!(f, "///         let custom_resource_definition = serde::Deserialize::deserialize(value)?;")?;
    writeln!(f, "///         Ok(custom_resource_definition)")?;
    writeln!(f, "///     }}),")?;
    writeln!(f, "///")?;
    writeln!(f, "///     k8s_if_ge_1_9!((CreateCustomResourceDefinitionResponse::Created(custom_resource_definition), _) =>")?;
    writeln!(f, "///         Ok(custom_resource_definition)),")?;
    writeln!(f, "///")?;
    writeln!(f, r#"///     (other, status_code) => Err(format!("unexpected response {{}} {{:?}}", status_code, other)),"#)?;
    writeln!(f, "/// }})?;")?;
    writeln!(f, "/// #")?;
    writeln!(f, "/// #     Ok(())")?;
    writeln!(f, "/// # }}")?;
    writeln!(f, "/// ```")?;
    writeln!(f, "#[macro_export] macro_rules! k8s_match {{")?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ }}) => {{")?;
    writeln!(f, "        match $test {{ $($arms)* }}")?;
    writeln!(f, "    }};")?;

    for v in MIN..=MAX {
        writeln!(f)?;

        for (name_suffix, enabled) in &[("", enabled_version == v), ("_ge", enabled_version >= v), ("_le", enabled_version <= v)] {
            writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ k8s_if{}_1_{}!($($arm:tt)*), $($rest:tt)* }}) => {{", name_suffix, v)?;
            if *enabled {
                writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($arm)*, $($rest)* }})")?;
            }
            else {
                writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* }} {{ $($rest)* }})")?;
            }
            writeln!(f, "    }};")?;
        }
    }

    writeln!(f)?;
    writeln!(f, "    (@inner {{ $test:expr }} {{ $($arms:tt)* }} {{ $next_pat:pat $(if $cond:expr)? => $next_expr:expr, $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ $($arms)* $next_pat $(if $cond)? => $next_expr, }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f)?;
    writeln!(f, "    ($test:expr, {{ $($rest:tt)* }}) => {{")?;
    writeln!(f, "        k8s_match!(@inner {{ $test }} {{ }} {{ $($rest)* }})")?;
    writeln!(f, "    }};")?;
    writeln!(f, "}}")?;

    Ok(())
}
