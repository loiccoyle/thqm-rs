use anyhow::Result;
// use http_auth_basic::Credentials;
// use tiny_http::{Header, Response};
use rouille::{router, start_server, Response};

use super::styles::Style;

pub fn start(style: &Style, address: &str) -> Result<()> {
    let rendered_template = style.render()?;

    start_server(
        address,
        move |request| {
            router!(request,
            (GET) (/) => {
                Response::html(rendered_template.to_owned())
            },
            (GET) (/select/{entry: String}) => {
                println!("{}", entry);
                Response::redirect_302("/")
            },
            _ => Response::empty_404()
            )
            /* println!("{:?}", request);
            let response = rouille::match_assets(&request, &style.style_path);
            if response.is_success() {
                return response;
            }
            Response::basic_http_auth_login_required("thqm") */
            // Response::text("test")
        },
    );
}
