use anyhow;
use cgi;
use serde_json;

mod data;

const JSON_FILE: &'static str = "cgi-bin/data.json";

cgi::cgi_try_main! { cgi_main }

fn cgi_main(_: cgi::Request) -> anyhow::Result<cgi::Response> {
    // 1. data.json読み込み
    let tables = data::load(JSON_FILE)?;

    // 2. レスポンス生成
    Ok(cgi::text_response(200, serde_json::to_string(&tables)?))
}
