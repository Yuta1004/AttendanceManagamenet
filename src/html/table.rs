use crate::data::{ Tables, Table, TableStat };

pub trait HTMLTableFormatter {
    fn format(&self) -> String;
}

impl HTMLTableFormatter for Tables {
    fn format(&self) -> String {
        let formatted_tables = self.tables
            .iter()
            .map(|table| table.format())
            .collect::<Vec<String>>()
            .join("\n");

        format!("
<table
    style=\"
        margin: auto;
    \"
>
    <tr>
        <th>名前</th>
        <th>状態</th>
        <th>コメント</th>
        <th>最終更新日時</th>
    </tr>
    {}
</table>
        ", formatted_tables)
    }
}

impl HTMLTableFormatter for Table {
    fn format(&self) -> String {
        format!("
<tr>
    <td>{}</td>
    <td>{}</td>
    <td>{}</td>
    <td>{}</td>
</tr>
        ", self.name, self.state.format(), self.comment, self.updated_at)
    }
}

impl HTMLTableFormatter for TableStat {
    fn format(&self) -> String {
        match self {
            TableStat::Occupied => "在席".to_string(),
            TableStat::Vacant => "空席".to_string(),
        }
    }
}
