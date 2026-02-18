// src/config.rs
type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn zenoh_client_config() -> Result<zenoh::Config, DynError> {
    Ok(zenoh::Config::from_json5(r#"
    {
      mode: "client",
      connect: { endpoints: ["tcp/zenoh:7447"] }
    }
    "#)?)
}