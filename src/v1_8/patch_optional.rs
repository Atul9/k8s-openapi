// Generated from definition io.k8s.PatchOptional

/// Common parameters for all patch operations.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PatchOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

impl PatchOptional<'_> {
    #[doc(hidden)]
    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]
    ///
    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.
    pub fn __serialize<T>(
        self,
        __query_pairs: &mut crate::url::form_urlencoded::Serializer<T>,
    ) where T: crate::url::form_urlencoded::Target {
        if let Some(pretty) = self.pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
    }
}
