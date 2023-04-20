mod data;
mod html;
mod request;

use anyhow;
use cgi;
use chrono::Local;

use data::Tables;
use html::{ HTMLRenderer, TopElem };
use request::Request;

const JSON_FILE: &'static str = "cgi-bin/data.json";

cgi::cgi_try_main! { cgi_main }

fn cgi_main(request: cgi::Request) -> anyhow::Result<cgi::Response> {
    // 1. data.json 読み込み
    let mut tables = data::load(JSON_FILE)?;

    // 2. リクエスト処理
    match request::parse(request)? {
        Request::View => {},
        Request::UpdateState(name) => {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S");
            tables.tables
                .iter_mut()
                .filter(|table| table.name == name)
                .for_each(|table| {
                    table.state = table.state.inverse();
                    table.updated_at = now.to_string();
                });
            tables.updated_at = now.to_string();
        }
    };

    // 3. data.json 書き込み
    data::store(JSON_FILE, &tables)?;

    // 4. レスポンス生成
    Ok(cgi::html_response(
        200,
        <Tables as HTMLRenderer<TopElem>>::render(&tables)
    ))
}
