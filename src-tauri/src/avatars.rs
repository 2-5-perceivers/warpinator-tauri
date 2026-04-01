use tauri::http::{Request, Response, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};
use warpinator_lib::remote_manager::RemoteManager;

pub fn avatars_protocol_handler<'a, R: Runtime>(
    ctx: UriSchemeContext<R>,
    request: Request<Vec<u8>>,
    responder: UriSchemeResponder,
) {
    let remote_manger = ctx.app_handle().state::<RemoteManager>().inner().clone();
    let remote_uuid = request.uri().authority().map(|a| a.as_str()).unwrap_or_default().to_string();

    tauri::async_runtime::spawn(async move {
        let remote = remote_manger.remote(remote_uuid.as_str()).await;
        let response = match remote {
            Some(remote) if remote.picture.is_some() => Response::builder()
                .header(header::CONTENT_TYPE, "image/png")
                .body(remote.picture.unwrap())
                .unwrap(),
            _ => Response::builder().status(404).body(Vec::new()).unwrap(),
        };

        responder.respond(response);
    });
}
