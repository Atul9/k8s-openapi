#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KubernetesAction {
	Connect,
	Delete,
	DeleteCollection,
	Get,
	List,
	Patch,
	Post,
	Proxy,
	Put,
	Watch,
	WatchList,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for KubernetesAction {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = KubernetesAction;

			fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "x-kubernetes-action")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
				Ok(match v {
					"connect" => KubernetesAction::Connect,
					"delete" => KubernetesAction::Delete,
					"deletecollection" => KubernetesAction::DeleteCollection,
					"get" => KubernetesAction::Get,
					"list" => KubernetesAction::List,
					"patch" => KubernetesAction::Patch,
					"post" => KubernetesAction::Post,
					"proxy" => KubernetesAction::Proxy,
					"put" => KubernetesAction::Put,
					"watch" => KubernetesAction::Watch,
					"watchlist" => KubernetesAction::WatchList,
					v => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)),
				})
			}
		}

		deserializer.deserialize_str(Visitor)
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
	Delete,
	Get,
	Patch,
	Post,
	Put,
}

#[derive(Clone, Debug)]
pub struct Operation {
	pub description: Option<String>,
	pub id: String,
	pub method: Method,
	pub kubernetes_action: Option<KubernetesAction>,
	pub kubernetes_group_kind_version: Option<super::KubernetesGroupKindVersion>,
	pub parameters: Vec<std::sync::Arc<Parameter>>,
	pub path: Path,
	pub responses: std::collections::BTreeMap<http::StatusCode, super::Schema>,
	pub tag: String,
}

#[derive(Clone, Debug)]
pub struct Parameter {
	pub location: ParameterLocation,
	pub name: String,
	pub required: bool,
	pub schema: super::Schema,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for Parameter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		#[derive(Debug, serde::Deserialize)]
		struct InnerParameter {
			description: Option<String>,
			#[serde(rename = "in")]
			location: String,
			name: String,
			required: Option<bool>,
			#[serde(rename = "type")]
			ty: Option<String>,
			schema: Option<super::Schema>,
		}

		let value: InnerParameter = serde::Deserialize::deserialize(deserializer)?;

		let (location, schema) = match (&*value.location, value.ty, value.schema) {
			("body", None, Some(schema)) => (
				ParameterLocation::Body,
				super::Schema {
					description: value.description,
					..schema
				},
			),

			("path", Some(ty), None) => (
				ParameterLocation::Path,
				super::Schema {
					description: value.description,
					kind: super::SchemaKind::Ty(super::Type::parse::<D>(&ty, None, None, None)?),
					kubernetes_group_kind_versions: None,
				},
			),

			("query", Some(ty), None) => (
				ParameterLocation::Query,
				super::Schema {
					description: value.description,
					kind: super::SchemaKind::Ty(super::Type::parse::<D>(&ty, None, None, None)?),
					kubernetes_group_kind_versions: None,
				},
			),

			_ => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&*value.location), &"a parameter location")),
		};

		let required = match (value.required, location) {
			(_, ParameterLocation::Path) => true,
			(Some(required), _) => required,
			(None, _) => false,
		};

		Ok(Parameter {
			location,
			name: value.name,
			required,
			schema,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterLocation {
	Body,
	Path,
	Query,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Path(pub String);

impl std::ops::Deref for Path {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl std::fmt::Display for Path {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
