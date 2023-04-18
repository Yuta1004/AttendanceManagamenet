use crate::data::{ Tables, Table, TablePos, TableStat };

trait HTMLFormatter {
    fn format(&self) -> String;
}

impl HTMLFormatter for Tables {
    fn format(&self) -> String {
        let wrap_table = |(idx, table): (usize, &Table)| {
            format!("<li>#{}</li>{}", idx, table.format())
        };

        let formatted_tables = self.tables
            .iter()
            .enumerate()
            .map(wrap_table)
            .collect::<Vec<String>>()
            .join("\n");

        format!("
<ul>{}</ul>
<p>最終更新日時: {}</p>
        ", formatted_tables, self.updated_at)
    }
}

impl HTMLFormatter for Table {
    fn format(&self) -> String {
        format!("
<ul>
    <li>名前: {}</li>
    <li>状態: {}</li>
    <li>コメント: {}</li>
    <li>最終更新日時: {}</li>
</ul>
        ", self.name, self.state.format(), self.comment, self.updated_at)
    }
}

impl HTMLFormatter for TableStat {
    fn format(&self) -> String {
        match self {
            TableStat::Occupied => "在席".to_string(),
            TableStat::Vacant => "空席".to_string(),
        }
    }
}

pub fn format(tables: &Tables) -> String {
    format!("
<html>
<head>
    <title> Attendance Management</title>
</head>
<body>
    <h1>出席情報</h1>
    <h2>座席表</h2>
    作成中
    <h2>座席情報</h2>
    {}
</body>
</html>
    ", tables.format())
}
