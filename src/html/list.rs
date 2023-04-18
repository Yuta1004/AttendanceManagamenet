use crate::data::{ Tables, Table, TableStat };

pub trait HTMLListFormatter {
    fn format(&self) -> String;
}

impl HTMLListFormatter for Tables {
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

impl HTMLListFormatter for Table {
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

impl HTMLListFormatter for TableStat {
    fn format(&self) -> String {
        match self {
            TableStat::Occupied => "在席".to_string(),
            TableStat::Vacant => "空席".to_string(),
        }
    }
}
