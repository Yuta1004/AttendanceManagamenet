use crate::data::{ Tables, Table, TableStat };

pub trait HTMLTableRenderer {
    fn render(&self) -> String;
}

impl HTMLTableRenderer for Tables {
    fn render(&self) -> String {
        let rendered_tables = self.tables
            .iter()
            .map(|table| table.render())
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
        ", rendered_tables)
    }
}

impl HTMLTableRenderer for Table {
    fn render(&self) -> String {
        format!("
<tr>
    <td>{}</td>
    <td>{}</td>
    <td>{}</td>
    <td>{}</td>
</tr>
        ", self.name, self.state.render(), self.comment, self.updated_at)
    }
}

impl HTMLTableRenderer for TableStat {
    fn render(&self) -> String {
        match self {
            TableStat::Occupied => "〇".to_string(),
            TableStat::Vacant => "".to_string(),
        }
    }
}

pub fn render<T>(elem: &T) -> String
where
    T: HTMLTableRenderer
{
    elem.render()
}
