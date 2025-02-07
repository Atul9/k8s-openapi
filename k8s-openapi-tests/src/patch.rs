k8s_if_1_8! {
	use k8s_openapi::http;
}
use k8s_openapi::serde_json;

use k8s_openapi::api::core::v1 as api;
k8s_if_1_8! {
	use k8s_openapi::api::apps::v1beta2 as apps;
}
k8s_if_ge_1_9! {
	use k8s_openapi::api::apps::v1 as apps;
}
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

#[test]
fn deployment() {
	crate::Client::with("patch-deployment", |client| {
		// Create deployment with container that uses alpine:3.6
		let deployment_spec = apps::DeploymentSpec {
			replicas: Some(1),
			selector: meta::LabelSelector {
				match_labels: Some(vec![
					("k8s-openapi-tests-patch-deployment-key".to_owned(), "k8s-openapi-tests-patch-deployment-value".to_owned()),
				].into_iter().collect()),
				..Default::default()
			},
			template: api::PodTemplateSpec {
				metadata: Some(meta::ObjectMeta {
					labels: Some(vec![
						("k8s-openapi-tests-patch-deployment-key".to_owned(), "k8s-openapi-tests-patch-deployment-value".to_owned()),
					].into_iter().collect()),
					..Default::default()
				}),
				spec: Some(api::PodSpec {
					containers: vec![
						api::Container {
							name: "k8s-openapi-tests-patch-deployment".to_owned(),
							image: "alpine:3.6".to_owned().into(),
							..Default::default()
						},
					],
					..Default::default()
				}),
				..Default::default()
			},
			..Default::default()
		};
		let deployment = apps::Deployment {
			metadata: Some(meta::ObjectMeta {
				name: Some("k8s-openapi-tests-patch-deployment".to_owned()),
				..Default::default()
			}),
			spec: Some(deployment_spec),
			..Default::default()
		};
		let (request, response_body) =
			apps::Deployment::create_namespaced_deployment("default", &deployment, Default::default())
			.expect("couldn't create deployment");
		{
			let response = client.execute(request).expect("couldn't create deployment");
			crate::get_single_value(response, response_body, |response, status_code| k8s_match!((response, status_code), {
				k8s_if_le_1_8!((apps::CreateNamespacedDeploymentResponse::Other(_), http::StatusCode::CREATED) =>
					Ok(crate::ValueResult::GotValue(()))),

				k8s_if_ge_1_9!((apps::CreateNamespacedDeploymentResponse::Created(_), _) =>
					Ok(crate::ValueResult::GotValue(()))),

				(other, status_code) => Err(format!("{:?} {}", other, status_code).into()),
			})).expect("couldn't create deployment");
		};


		// Use JSON patch to patch deployment with alpine:3.7 container
		let patch = meta::Patch::Json(vec![
			serde_json::Value::Object(vec![
				("op".to_owned(), serde_json::Value::String("test".to_owned())),
				("path".to_owned(), serde_json::Value::String("/spec/template/spec/containers/0/image".to_owned())),
				("value".to_owned(), serde_json::Value::String("alpine:3.6".to_owned())),
			].into_iter().collect()),
			serde_json::Value::Object(vec![
				("op".to_owned(), serde_json::Value::String("replace".to_owned())),
				("path".to_owned(), serde_json::Value::String("/spec/template/spec/containers/0/image".to_owned())),
				("value".to_owned(), serde_json::Value::String("alpine:3.7".to_owned())),
			].into_iter().collect()),
		]);
		patch_and_assert_container_has_image(client, &patch, "alpine:3.7");


		// Use merge patch to patch deployment with alpine:3.8 container
		let patch = apps::Deployment {
			spec: Some(apps::DeploymentSpec {
				template: api::PodTemplateSpec {
					spec: Some(api::PodSpec {
						containers: vec![
							api::Container {
								name: "k8s-openapi-tests-patch-deployment".to_owned(),
								image: "alpine:3.8".to_owned().into(),
								..Default::default()
							},
						],
						..Default::default()
					}),
					..Default::default()
				},
				..Default::default()
			}),
			..Default::default()
		};
		let patch = meta::Patch::Merge(serde_json::to_value(&patch).expect("couldn't create patch"));
		patch_and_assert_container_has_image(client, &patch, "alpine:3.8");


		// Use strategic merge patch to patch deployment with alpine:3.9 container
		let patch = apps::Deployment {
			spec: Some(apps::DeploymentSpec {
				template: api::PodTemplateSpec {
					spec: Some(api::PodSpec {
						containers: vec![
							api::Container {
								name: "k8s-openapi-tests-patch-deployment".to_owned(),
								image: "alpine:3.9".to_owned().into(),
								..Default::default()
							},
						],
						..Default::default()
					}),
					..Default::default()
				},
				..Default::default()
			}),
			..Default::default()
		};
		let patch = meta::Patch::StrategicMerge(serde_json::to_value(&patch).expect("couldn't create patch"));
		patch_and_assert_container_has_image(client, &patch, "alpine:3.9");


		// Delete deployment
		let (request, response_body) =
			apps::Deployment::delete_namespaced_deployment("k8s-openapi-tests-patch-deployment", "default", Default::default())
			.expect("couldn't delete deployment");
		{
			let response = client.execute(request).expect("couldn't delete deployment");
			crate::get_single_value(response, response_body, |response, status_code| match response {
				apps::DeleteNamespacedDeploymentResponse::OkStatus(_) | apps::DeleteNamespacedDeploymentResponse::OkValue(_) => Ok(crate::ValueResult::GotValue(())),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't delete deployment");
		}

		// Delete all pods of the deployment using label selector
		let (request, response_body) =
			api::Pod::delete_collection_namespaced_pod(
				"default",
				Default::default(),
				k8s_openapi::ListOptional {
					label_selector: Some("k8s-openapi-tests-patch-deployment-key=k8s-openapi-tests-patch-deployment-value"),
					..Default::default()
				},
			)
			.expect("couldn't delete pods collection");
		{
			let response = client.execute(request).expect("couldn't delete pods collection");
			crate::get_single_value(response, response_body, |response, status_code| match response {
				api::DeleteCollectionNamespacedPodResponse::OkStatus(_) | api::DeleteCollectionNamespacedPodResponse::OkValue(_) => Ok(crate::ValueResult::GotValue(())),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't delete pods collection");
		}
	});
}

/// Patch the deployment with the given path, and assert that the patched deployment has a container with the expected image
fn patch_and_assert_container_has_image(client: &mut crate::Client, patch: &meta::Patch, expected_image: &str) {
	let (request, response_body) =
		apps::Deployment::patch_namespaced_deployment("k8s-openapi-tests-patch-deployment", "default", &patch, Default::default())
		.expect("couldn't create patch");
	println!("{:?}", request);

	let response = client.execute(request).expect("couldn't patch deployment");

	let deployment =
		crate::get_single_value(response, response_body, |response, status_code| match response {
			apps::PatchNamespacedDeploymentResponse::Ok(deployment) => Ok(crate::ValueResult::GotValue(deployment)),
			other => {
				// Err(format!("{:?} {}", other, status_code).into())
				println!("{:?} {}", other, status_code);
				Ok(crate::ValueResult::NeedMoreData)
			},
		}).expect("couldn't patch deployment");

	let image =
		deployment
		.spec.expect("couldn't get deployment spec")
		.template
		.spec.expect("couldn't get pod spec")
		.containers
		.into_iter()
		.next().expect("couldn't get container spec")
		.image.expect("couldn't get image from container spec");

	assert_eq!(image, expected_image);
}
