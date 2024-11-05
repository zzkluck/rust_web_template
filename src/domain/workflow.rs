use crate::domain::ruoyi_prompt_request::ComfyNodeInput;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Workflow {
    prompt: HashMap<String, WorkflowNode>,
    pub client_id: String,
    #[serde(skip)]
    title_map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowNode {
    pub inputs: Value,
    class_type: String,
    _meta: WorkflowMeta,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowMeta {
    title: String,
}

impl Workflow {
    pub fn get_workflow_by_id(workflow_id: &str) -> Self {
        let _ = workflow_id;
        let mut workflow =
            serde_json::from_str::<Workflow>(include_str!("../../assets/comfy_post_prompt.json"))
                .expect("Failed to parse workflow.");
        workflow._gen_hash_map();
        workflow
    }

    fn _gen_hash_map(&mut self) {
        self.title_map = self
            .prompt
            .iter()
            .map(|(k, v)| (v._meta.title.clone(), k.clone()))
            .collect()
    }

    pub fn set_workflow(&mut self, settings: Vec<ComfyNodeInput>) {
        settings.iter().for_each(|input| {
            self.prompt
                .get_mut(self.title_map[input.node_title].as_str())
                .expect("title not exist")
                .inputs[input.key] = serde_json::json!(input.value)
        })
    }

    pub fn get_setting(&self, title: &str, key: &str) -> &str {
        self.prompt[self.title_map[title].as_str()].inputs[key]
            .as_str()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_serialization_from_json_should_success() {
        let workflow =
            serde_json::from_str::<Workflow>(include_str!("../../assets/comfy_post_prompt.json"))
                .expect("Failed to parse example json");

        assert_eq!(workflow.prompt.len(), 17)
    }

    #[test]
    fn get_workflow_by_id_should_success_and_auto_gen_map() {
        let workflow = Workflow::get_workflow_by_id("test");

        assert_eq!(workflow.title_map.len(), 17);
        assert_eq!(workflow.title_map["🛫k-ε湍流模型"], "50");
    }

    #[test]
    fn set_workflow_success() {
        let mut workflow = Workflow::get_workflow_by_id("test");
        assert_eq!(
            workflow.get_setting("🛫速度入口条件", "inlet_velocity"),
            "uniform (10 0 0)"
        );
        workflow.set_workflow(vec![ComfyNodeInput {
            node_title: "🛫速度入口条件",
            key: "inlet_velocity",
            value: "uniform (11 45 14)",
        }]);
        assert_eq!(
            workflow.get_setting("🛫速度入口条件", "inlet_velocity"),
            "uniform (11 45 14)"
        );
    }
}
