use serde::Serialize;

pub trait KubeResource<T: Serialize> {
    fn get_kube_object(&self) -> &T;

    fn resource_to_yaml(&self) -> String {
        match serde_yaml::to_string(&self.get_kube_object()) {
            Ok(yaml) => yaml,
            Err(_) => "".into(),
        }
    }
}
