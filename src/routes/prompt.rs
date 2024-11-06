use crate::comfy_client::ComfyClient;
use crate::domain::RuoyiPromptRequest;
use crate::domain::Workflow;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::MySqlPool;

pub async fn prompt(
    prompt: web::Json<RuoyiPromptRequest>,
    db_pool: web::Data<MySqlPool>,
    comfy_client: web::Data<ComfyClient>,
) -> impl Responder {
    let mut workflow = Workflow::get_workflow_by_id(&prompt.workflow_id);
    let ruoyi_config_request = prompt.to_node_input_list();
    workflow.set_workflow(ruoyi_config_request);
    if let Some(client_id) = &prompt.client_id {
        workflow.client_id = format!("{}-{}", client_id, comfy_client.client_id);
    }
    let task_name = match &prompt.task_name {
        None => "anonymous_task".to_string(),
        Some(name) => name.clone(),
    };
    let response = comfy_client
        .post_prompt(workflow)
        .await
        .expect("Post prompt failed");

    sqlx::query!(
        r#"
        INSERT INTO Prompt (name, state, create_time, prompt_id)
        VALUES (?, ?, ?, ?)
        "#,
        task_name,
        0,
        Utc::now(),
        response.prompt_id
    )
    .execute(db_pool.get_ref())
    .await
    .expect("Failed to run query");

    HttpResponse::Ok()
}
