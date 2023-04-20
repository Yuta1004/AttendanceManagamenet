use thiserror::Error;

use percent_encoding::percent_decode;

pub enum Request {
    View,
    UpdateState(String),
}

#[derive(Debug, Error)]
enum RequestError {
    #[error("UTF-8 Decoding error!")]
    UTF8DecodingError,
}

pub fn parse(request: cgi::Request) -> anyhow::Result<Request> {
    let req_body = match percent_decode(request.body()).decode_utf8() {
        Ok(body) => Ok(body.to_string()),
        Err(_) => Err(RequestError::UTF8DecodingError),
    }?;

    let form_elements = req_body.split(";");
    for elem in form_elements.map(|item| item.split("=").collect::<Vec<&str>>()) {
        match elem[0] {
            "state" => return Ok(Request::UpdateState(elem[1].to_string())),
            _ => {}
        }
    }

    Ok(Request::View)
}
