use std::fs::File;
use std::io::BufReader;

use anyhow;
use serde::{ Serialize, Deserialize };
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tables {
    pub tables: Vec<Table>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub pos: TablePos,
    pub state: TableStat,
    pub comment: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TablePos {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
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
