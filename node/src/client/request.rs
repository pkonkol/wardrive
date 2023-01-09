use ureq::{Agent, AgentBuilder};

pub fn get_status() -> String {
    let agent = ureq::AgentBuilder::new().build();

    let body = (agent.get("http://localhost:8080/api/status")).call()?.into_string();
    body
}

pub fn post_update(s: String) -> Result<(), > {
    let agent = ureq::AgentBuilder::new().build();

    let response = agent.post("http://localhost:8080/api/update")
                            .send_string(r#"{"Jan": "Papiez"}"#)?
                            .into_string()?;
}