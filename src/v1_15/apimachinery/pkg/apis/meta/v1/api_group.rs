// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup

/// APIGroup contains the name, the supported versions, and the preferred version of a group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIGroup {
    /// name is the name of the group.
    pub name: String,

    /// preferredVersion is the version preferred by the API server, which probably is the storage version.
    pub preferred_version: Option<crate::v1_15::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,

    /// a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    pub server_address_by_client_cidrs: Option<Vec<crate::v1_15::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>>,

    /// versions are the versions supported in this group.
    pub versions: Vec<crate::v1_15::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,
}

impl crate::Resource for APIGroup {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "APIGroup"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl<'de> serde::Deserialize<'de> for APIGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_preferred_version,
            Key_server_address_by_client_cidrs,
            Key_versions,
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
                            "name" => Field::Key_name,
                            "preferredVersion" => Field::Key_preferred_version,
                            "serverAddressByClientCIDRs" => Field::Key_server_address_by_client_cidrs,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = APIGroup;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct APIGroup")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_preferred_version: Option<crate::v1_15::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery> = None;
                let mut value_server_address_by_client_cidrs: Option<Vec<crate::v1_15::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>> = None;
                let mut value_versions: Option<Vec<crate::v1_15::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>> = None;

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
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_preferred_version => value_preferred_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_server_address_by_client_cidrs => value_server_address_by_client_cidrs = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIGroup {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    preferred_version: value_preferred_version,
                    server_address_by_client_cidrs: value_server_address_by_client_cidrs,
                    versions: value_versions.ok_or_else(|| serde::de::Error::missing_field("versions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIGroup",
            &[
                "apiVersion",
                "kind",
                "name",
                "preferredVersion",
                "serverAddressByClientCIDRs",
                "versions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for APIGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIGroup",
            4 +
            self.preferred_version.as_ref().map_or(0, |_| 1) +
            self.server_address_by_client_cidrs.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.preferred_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "preferredVersion", value)?;
        }
        if let Some(value) = &self.server_address_by_client_cidrs {
            serde::ser::SerializeStruct::serialize_field(&mut state, "serverAddressByClientCIDRs", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "versions", &self.versions)?;
        serde::ser::SerializeStruct::end(state)
    }
}
