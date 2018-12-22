// Generated from definition io.k8s.api.rbac.v1alpha1.ClusterRoleBinding

/// ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRoleBinding {
    /// Standard object's metadata.
    pub metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// RoleRef can only reference a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
    pub role_ref: crate::v1_9::api::rbac::v1alpha1::RoleRef,

    /// Subjects holds references to the objects the role applies to.
    pub subjects: Vec<crate::v1_9::api::rbac::v1alpha1::Subject>,
}

// Begin rbac.authorization.k8s.io/v1alpha1/ClusterRoleBinding

// Generated from operation createRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// create a ClusterRoleBinding
    ///
    /// Use [`CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_rbac_authorization_v1alpha1_cluster_role_binding(
        body: &crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding,
        optional: CreateRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::create_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.create_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct CreateRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::create_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.create_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    Ok(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Created(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Accepted(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Accepted(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// delete a ClusterRoleBinding
    ///
    /// Use [`DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_rbac_authorization_v1alpha1_cluster_role_binding(
        name: &str,
        optional: DeleteRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            grace_period_seconds,
            orphan_dependents,
            pretty,
            propagation_policy,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::delete_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.delete_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct DeleteRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    pub grace_period_seconds: Option<i64>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    pub orphan_dependents: Option<bool>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    pub propagation_policy: Option<&'a str>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::delete_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.delete_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1CollectionClusterRoleBinding

impl ClusterRoleBinding {
    /// delete collection of ClusterRoleBinding
    ///
    /// Use [`DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse`](./enum.DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn delete_rbac_authorization_v1alpha1_collection_cluster_role_binding(
        optional: DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::delete_rbac_authorization_v1alpha1_collection_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.delete_rbac_authorization_v1alpha1_collection_cluster_role_binding)
#[derive(Debug, Default)]
pub struct DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::delete_rbac_authorization_v1alpha1_collection_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.delete_rbac_authorization_v1alpha1_collection_cluster_role_binding)
#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse {
    OkStatus(crate::v1_9::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::UNAUTHORIZED => Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation listRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// list or watch objects of kind ClusterRoleBinding
    ///
    /// Use [`ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn list_rbac_authorization_v1alpha1_cluster_role_binding(
        optional: ListRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ListRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::list_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.list_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct ListRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::list_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.list_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    Ok(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBindingList),
    Unauthorized,
    Other,
}

impl crate::Response for ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation patchRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// partially update the specified ClusterRoleBinding
    ///
    /// Use [`PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn patch_rbac_authorization_v1alpha1_cluster_role_binding(
        name: &str,
        body: &crate::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        optional: PatchRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let PatchRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::patch(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::patch_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.patch_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct PatchRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::patch_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.patch_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    Ok(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation readRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// read the specified ClusterRoleBinding
    ///
    /// Use [`ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn read_rbac_authorization_v1alpha1_cluster_role_binding(
        name: &str,
        optional: ReadRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReadRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::read_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.read_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct ReadRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::read_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.read_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    Ok(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// replace the specified ClusterRoleBinding
    ///
    /// Use [`ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn replace_rbac_authorization_v1alpha1_cluster_role_binding(
        name: &str,
        body: &crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding,
        optional: ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::put(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::replace_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.replace_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::replace_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.replace_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    Ok(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Created(crate::v1_9::api::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized,
    Other,
}

impl crate::Response for ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Created(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRoleBinding

impl ClusterRoleBinding {
    /// watch changes to an object of kind ClusterRoleBinding
    ///
    /// Use [`WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse`](./enum.WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ClusterRoleBinding
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_rbac_authorization_v1alpha1_cluster_role_binding(
        name: &str,
        optional: WatchRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchRbacAuthorizationV1alpha1ClusterRoleBindingOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings/{name}?", name = name);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::watch_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.watch_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug, Default)]
pub struct WatchRbacAuthorizationV1alpha1ClusterRoleBindingOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::watch_rbac_authorization_v1alpha1_cluster_role_binding`](./struct.ClusterRoleBinding.html#method.watch_rbac_authorization_v1alpha1_cluster_role_binding)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other, 0)),
        }
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRoleBindingList

impl ClusterRoleBinding {
    /// watch individual changes to a list of ClusterRoleBinding
    ///
    /// Use [`WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse`](./enum.WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn watch_rbac_authorization_v1alpha1_cluster_role_binding_list(
        optional: WatchRbacAuthorizationV1alpha1ClusterRoleBindingListOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let WatchRbacAuthorizationV1alpha1ClusterRoleBindingListOptional {
            continue_,
            field_selector,
            include_uninitialized,
            label_selector,
            limit,
            pretty,
            resource_version,
            timeout_seconds,
            watch,
        } = optional;
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings?");
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`ClusterRoleBinding::watch_rbac_authorization_v1alpha1_cluster_role_binding_list`](./struct.ClusterRoleBinding.html#method.watch_rbac_authorization_v1alpha1_cluster_role_binding_list)
#[derive(Debug, Default)]
pub struct WatchRbacAuthorizationV1alpha1ClusterRoleBindingListOptional<'a> {
    /// The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    pub continue_: Option<&'a str>,
    /// A selector to restrict the list of returned objects by their fields. Defaults to everything.
    pub field_selector: Option<&'a str>,
    /// If true, partially initialized resources are included in the response.
    pub include_uninitialized: Option<bool>,
    /// A selector to restrict the list of returned objects by their labels. Defaults to everything.
    pub label_selector: Option<&'a str>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    pub limit: Option<i64>,
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
    /// When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    pub resource_version: Option<&'a str>,
    /// Timeout for the list/watch call.
    pub timeout_seconds: Option<i64>,
    /// Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub watch: Option<bool>,
}

/// Parses the HTTP response of [`ClusterRoleBinding::watch_rbac_authorization_v1alpha1_cluster_role_binding_list`](./struct.ClusterRoleBinding.html#method.watch_rbac_authorization_v1alpha1_cluster_role_binding_list)
#[derive(Debug)]
pub enum WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse {
    Ok(crate::v1_9::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl crate::Response for WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse::Ok(result), byte_offset))
            },
            http::StatusCode::UNAUTHORIZED => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse::Unauthorized, 0)),
            _ => Ok((WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse::Other, 0)),
        }
    }
}

// End rbac.authorization.k8s.io/v1alpha1/ClusterRoleBinding

impl crate::Resource for ClusterRoleBinding {
    fn api_version() -> &'static str {
        "rbac.authorization.k8s.io/v1alpha1"
    }

    fn group() -> &'static str {
        "rbac.authorization.k8s.io"
    }

    fn kind() -> &'static str {
        "ClusterRoleBinding"
    }

    fn version() -> &'static str {
        "v1alpha1"
    }
}

impl crate::Metadata for ClusterRoleBinding {
    type Ty = crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for ClusterRoleBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_role_ref,
            Key_subjects,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "roleRef" => Field::Key_role_ref,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRoleBinding;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct ClusterRoleBinding")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_role_ref: Option<crate::v1_9::api::rbac::v1alpha1::RoleRef> = None;
                let mut value_subjects: Option<Vec<crate::v1_9::api::rbac::v1alpha1::Subject>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_role_ref => value_role_ref = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subjects => value_subjects = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRoleBinding {
                    metadata: value_metadata,
                    role_ref: value_role_ref.ok_or_else(|| serde::de::Error::missing_field("roleRef"))?,
                    subjects: value_subjects.ok_or_else(|| serde::de::Error::missing_field("subjects"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterRoleBinding",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "roleRef",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ClusterRoleBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterRoleBinding",
            0 +
            2 +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1 +
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "roleRef", &self.role_ref)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        serde::ser::SerializeStruct::end(state)
    }
}
