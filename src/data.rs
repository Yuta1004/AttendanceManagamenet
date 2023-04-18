use std::fs::File;
use std::io::BufReader;

use anyhow;
use serde::{ Serialize, Deserialize };
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Tables {
    tables: Vec<Table>,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    name: String,
    pos: TablePos,
    state: TableStat,
    comment: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TablePos {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Serialize, Deserialize)]
pub enum TableStat {
    Occupied,   // 出席
    Vacant,     // 空席
}

pub fn load(fname: &str) -> anyhow::Result<Tables> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}
