use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub struct StartServerPayload {
  pub port: u16,
  pub ip: String,
  pub user_name: String,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  StartServerCommand {
    callback: String,
    error: String,
    payload: StartServerPayload,
  },
}
