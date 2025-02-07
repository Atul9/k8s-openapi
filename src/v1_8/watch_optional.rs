// Generated from definition io.k8s.WatchOptional

/// Common parameters for all watch operations.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WatchOptional<'a> {
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,

    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,

    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,

    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,

    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,

    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
}

impl WatchOptional<'_> {
    #[doc(hidden)]
    /// Serializes this object to a [`crate::url::form_urlencoded::Serializer`]
    ///
    /// This function is only exposed for use by the `k8s-openapi-derive` crate and is not part of the stable public API.
    pub fn __serialize<T>(
        self,
        __query_pairs: &mut crate::url::form_urlencoded::Serializer<T>,
    ) where T: crate::url::form_urlencoded::Target {
        if let Some(field_selector) = self.field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = self.include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = self.label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(pretty) = self.pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = self.resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = self.timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        __query_pairs.append_pair("watch", "true");
    }
}
