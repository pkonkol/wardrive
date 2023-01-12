use ureq::{Agent, AgentBuilder};

pub fn get_status() -> Result<String, ureq::Error> {
    let agent = ureq::AgentBuilder::new().build();

    let body = (agent.get("http://localhost:8080/api/status")).call()?.into_string()?;
    Ok(body)
}

pub fn post_update(s: String) -> Result<String, ureq::Error> {
    let agent = ureq::AgentBuilder::new().build();

    let response = agent.post("http://localhost:8080/api/update")
                            .send_string(r#"[{"Jan": "Papiez"}, {"karol": ["wojtyla", 2]}]"#)?
                            .into_string()?;
    Ok(response)
}