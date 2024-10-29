use std::collections::HashSet;
use std::str::FromStr;
use actix_web::{HttpResponse, Responder, web};
use serde_json::Value;
use crate::configuration::Settings;

pub fn queue(prompt_id: web::Query<String>, config: web::Data<Settings>) -> impl Responder {
    HttpResponse::ImATeapot()
}


#[derive(Debug)]
struct ComfyQueueInfo {
    task_id: u64,
    prompt_id: String
}

#[derive(Debug)]
struct ComfyQueueResponse {
    running: Vec<ComfyQueueInfo>,
    pending: Vec<ComfyQueueInfo>
}

impl ComfyQueueResponse {
    fn prompt_exists(&self, prompt_id: &str) -> bool {
        self.running
            .iter()
            .chain(self.pending.iter())
            .map(|x| x.prompt_id.as_str())
            .collect::<HashSet<&str>>()
            .contains(prompt_id)
    }
}

fn _parse_response_array(json: &Value, queue_name: &str) -> Vec<ComfyQueueInfo> {
    json.get(queue_name)
        .expect(&format!("<{}> not in json object.", queue_name))
        .as_array()
        .expect(&format!("<{}> is not an array.", queue_name))
        .iter()
        .map(|x| {
            if x.as_array()
                .expect(&format!("Obj {:?} in {} is not an array.", x, queue_name))
                .len() != 5 {
                panic!("Obj {:?} in {} have an invalid length. Expect: 5", x, queue_name)
            }

            // two index op is safe here.
            ComfyQueueInfo {
                task_id: x[0].as_u64().unwrap(),
                prompt_id: x[1].as_str().unwrap().to_string()
            }
        })
        .collect()
}


impl FromStr for ComfyQueueResponse {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json: Value = serde_json::from_str(s)?;
        Ok(ComfyQueueResponse {
            running: _parse_response_array(&json, "queue_running"),
            pending: _parse_response_array(&json, "queue_pending"),
        })
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::routes::queue::ComfyQueueResponse;

    #[test]
    fn response_json_can_be_parsed() {
        let response =
            ComfyQueueResponse::from_str(include_str!("../../assets/comfy_queue_response.json"));
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.running[0].task_id, 0);
        assert_eq!(response.running[0].prompt_id, "098ac3a7-a2d2-4dc6-b562-fbd51191c4f6");
        assert_eq!(response.pending[1].task_id, 2);
        assert_eq!(response.pending[2].prompt_id, "b71ff4ba-073b-4e6c-ae73-5efd851cbb0b");

    }

    #[test]
    fn response_parse_reject_empty_json() {
        let response = ComfyQueueResponse::from_str("");
        assert!(response.is_err());
    }

    #[test]
    fn prompt_exists_normal() {
        let response =
            ComfyQueueResponse::from_str(include_str!("../../assets/comfy_queue_response.json"));
        assert!(response.unwrap().prompt_exists("098ac3a7-a2d2-4dc6-b562-fbd51191c4f6"));
    }

    #[test]
    fn prompt_not_exists() {
        let response =
            ComfyQueueResponse::from_str(include_str!("../../assets/comfy_queue_response.json"));
        assert!(!response.unwrap().prompt_exists("zzkluck-is-not-exists"));
    }
}