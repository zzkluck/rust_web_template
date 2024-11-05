use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct RuoyiPromptRequest {
    pub workflow_id: String,
    pub client_id: Option<String>,
    pub task_name: Option<String>,
    args: Value,
}

impl RuoyiPromptRequest {
    pub fn to_node_input_list(&self) -> Vec<ComfyNodeInput> {
        self.args
            .as_object()
            .expect("args should be a map")
            .into_iter()
            .flat_map(|(title, settings)| {
                settings
                    .as_object()
                    .expect("setting should be key/value pairs")
                    .into_iter()
                    .map(|(k, v)| ComfyNodeInput {
                        node_title: title,
                        key: k,
                        value: v.as_str().expect("value should be string."),
                    })
            })
            .collect::<Vec<ComfyNodeInput>>()
    }
}

pub struct ComfyNodeInput<'obj> {
    pub node_title: &'obj str,
    pub key: &'obj str,
    pub value: &'obj str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_request_to_node_input_list_should_success() {
        let prompt_request = serde_json::from_str::<RuoyiPromptRequest>(include_str!(
            "../../assets/ruoyi_post_prompt.json"
        ))
        .expect("Failed to parse example json");
        assert_eq!(prompt_request.workflow_id, "1");
        let inputs = prompt_request.to_node_input_list();
        assert_eq!(inputs.len(), 8);

        let input = inputs
            .iter()
            .find(|i| i.node_title == "求解控制器器" && i.key == "solver")
            .expect("Failed to find expected input.");

        assert_eq!(input.value, "incompressibleFluid");
    }
}
