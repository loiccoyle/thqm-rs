use anyhow::{anyhow, Result};
use log::*;
use rouille::{input, match_assets, router, Request, Response, Server};
use std::process::exit;

use crate::styles::Style;

/// Starts the rouille server.
pub fn start(
    style: &Style,
    address: &str,
    oneshot: bool,
    login: Option<String>,
    password: Option<String>,
) -> Result<()> {
    let rendered_template = style.render()?;
    let style_base_path = style
        .base_path
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "Could not convert style base path {:?} to str.",
                style.base_path
            )
        })?
        .to_owned();

    let server = Server::new(address, move |request| {
        if login.is_some() && password.is_some() {
            if let Some(rep) =
                handle_auth(request, login.as_ref().unwrap(), password.as_ref().unwrap())
            {
                return rep;
            }
        }

        router!(request,
        (GET) (/) => {
            if let Some(command) = request.get_param("cmd") {
                handle_cmd(command)
            } else if let Some(entry) = request.get_param("select") {
                handle_select(entry, oneshot)
            } else {
                Response::html(&rendered_template)
            }
        },
        (GET) (/select/{entry: String}) => {
            handle_select(entry, oneshot)
        },
        (GET) (/cmd/{command: String}) => {
            handle_cmd(command)
        },
        _ => {
            let response = match_assets(request, &style_base_path);
            if response.is_success() {
                response
            } else {
                Response::empty_404()
            }
        }
        )
    })
    .expect("Failed to init the server");
    server.run();
    Ok(())
}

/// Handles the selection logic.
/// Prints the selected `entry` to stdout and redirects to '/'.
/// If `oneshot`, the server exits.
pub fn handle_select(entry: String, oneshot: bool) -> Response {
    println!("{}", entry);
    if oneshot {
        exit(0);
    }
    Response::redirect_302("/")
}

/// Handles the cmd logic.
/// If `cmd` is not recognized, returns a 404 response.
pub fn handle_cmd(command: String) -> Response {
    match command.as_str() {
        "shutdown" => {
            exit(0);
        }
        _ => Response::empty_404(),
    }
}

/// Handles the authentication.
pub fn handle_auth(request: &Request, login: &str, password: &str) -> Option<Response> {
    let auth = match input::basic_http_auth(request) {
        Some(a) => a,
        None => return Some(Response::basic_http_auth_login_required("thqm")),
    };
    debug!("Handling auth: {:?}", auth);

    if auth.login != login || auth.password != password {
        Some(Response::basic_http_auth_login_required("thqm"))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_handle_select() {
        let response = handle_select("test".to_string(), false);
        assert_eq!(response.status_code, 302);
    }

    #[test]
    fn test_handle_cmd() {
        let response = handle_cmd("missing_cmd".to_string());
        assert_eq!(response.status_code, 404);
    }

    #[test]
    fn test_handle_auth() {
        let req = rouille::Request::fake_http("GET", "/", vec![], vec![]);
        let resp = handle_auth(&req, "thqm", "test");
        assert!(resp.is_some());

        let req = rouille::Request::fake_http(
            "GET",
            "/",
            vec![(
                "Authorization".to_string(),
                "Basic dGhxbTp0ZXN0".to_string(), // thqm, test
            )],
            vec![],
        );
        let resp = handle_auth(&req, "thqm", "test");
        assert!(resp.is_none());
    }
}
