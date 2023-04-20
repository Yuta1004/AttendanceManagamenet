use anyhow;
use cgi;

mod data;
mod html;

const JSON_FILE: &'static str = "cgi-bin/data.json";

cgi::cgi_try_main! { cgi_main }

fn cgi_main(_: cgi::Request) -> anyhow::Result<cgi::Response> {
    // 1. data.json読み込み
    let tables = data::load(JSON_FILE)?;

    // 2. レスポンス生成
    Ok(cgi::html_response(200, html::render(&tables)))
}
