// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

mod cmd;

use std::fmt::Display;

fn start_server() -> Result<(), ()> {
  return Ok(());
}

#[derive(Debug)]
struct CommandError {}
impl std::error::Error for CommandError {}
impl Display for CommandError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", "")
  }
}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            StartServerCommand {
              callback,
              error,
              payload,
            } => {
              //  your command code
              println!(
                "Server Start {} {} {}",
                payload.ip, payload.user_name, payload.port
              );

              let result = start_server();

              tauri::execute_promise(
                _webview,
                move || match result {
                  Ok(_) => Ok(cmd::StartServerPayload {
                    port: 777,
                    ip: "".to_string(),
                    user_name: "".to_string(),
                  }),
                  Err(_) => Err(CommandError {}.into()),
                },
                callback,
                error,
              );
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
