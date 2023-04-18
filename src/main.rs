use cgi;

cgi::cgi_try_main! { entrypoint }

fn entrypoint(_: cgi::Request) -> Result<cgi::Response, String> {
    let greeting = "hello world".to_string();
    Ok(cgi::text_response(200, greeting))
}
