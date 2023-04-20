mod data;
mod html;
mod request;

use anyhow;
use cgi;

use request::Request;

const JSON_FILE: &'static str = "cgi-bin/data.json";

cgi::cgi_try_main! { cgi_main }

fn cgi_main(request: cgi::Request) -> anyhow::Result<cgi::Response> {
    // 1. data.json 読み込み
    let mut tables = data::load(JSON_FILE)?;

    // 2. リクエスト処理
    match request::parse(request)? {
        Request::View => {},
        Request::UpdateState(name) => tables.tables
            .iter_mut()
            .filter(|table| table.name == name)
            .for_each(|table| table.state = table.state.inverse()),
    };

    // 3. data.json 書き込み
    data::store(JSON_FILE, &tables)?;

    // 3. レスポンス生成
    Ok(cgi::html_response(200, html::render(&tables)))
}
