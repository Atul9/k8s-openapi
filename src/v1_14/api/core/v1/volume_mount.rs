// Generated from definition io.k8s.api.core.v1.VolumeMount

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mount_path: String,

    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    pub mount_propagation: Option<String>,

    /// This must match the Name of a Volume.
    pub name: String,

    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    pub read_only: Option<bool>,

    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    pub sub_path: Option<String>,

    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive. This field is alpha in 1.14.
    pub sub_path_expr: Option<String>,
}

impl<'de> serde::Deserialize<'de> for VolumeMount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_mount_path,
            Key_mount_propagation,
            Key_name,
            Key_read_only,
            Key_sub_path,
            Key_sub_path_expr,
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
                            "mountPath" => Field::Key_mount_path,
                            "mountPropagation" => Field::Key_mount_propagation,
                            "name" => Field::Key_name,
                            "readOnly" => Field::Key_read_only,
                            "subPath" => Field::Key_sub_path,
                            "subPathExpr" => Field::Key_sub_path_expr,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VolumeMount;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct VolumeMount")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_mount_path: Option<String> = None;
                let mut value_mount_propagation: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_sub_path: Option<String> = None;
                let mut value_sub_path_expr: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mount_path => value_mount_path = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_mount_propagation => value_mount_propagation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only => value_read_only = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sub_path => value_sub_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_sub_path_expr => value_sub_path_expr = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeMount {
                    mount_path: value_mount_path.ok_or_else(|| serde::de::Error::missing_field("mountPath"))?,
                    mount_propagation: value_mount_propagation,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    read_only: value_read_only,
                    sub_path: value_sub_path,
                    sub_path_expr: value_sub_path_expr,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeMount",
            &[
                "mountPath",
                "mountPropagation",
                "name",
                "readOnly",
                "subPath",
                "subPathExpr",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for VolumeMount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeMount",
            2 +
            self.mount_propagation.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.sub_path.as_ref().map_or(0, |_| 1) +
            self.sub_path_expr.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "mountPath", &self.mount_path)?;
        if let Some(value) = &self.mount_propagation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "mountPropagation", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.read_only {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.sub_path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "subPath", value)?;
        }
        if let Some(value) = &self.sub_path_expr {
            serde::ser::SerializeStruct::serialize_field(&mut state, "subPathExpr", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
