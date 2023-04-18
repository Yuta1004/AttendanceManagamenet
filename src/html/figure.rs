use crate::data::{ Tables, Table };

pub trait HTMLFigureFormatter {
    fn format(&self) -> String;
}

impl HTMLFigureFormatter for Tables {
    fn format(&self) -> String {
        let formatted_tables = self.tables
            .iter()
            .map(|table| table.format())
            .collect::<Vec<String>>()
            .join("\n");

        format!("
<div
    style=\"
        border: solid;
        position: relative;
        width: 80vw;
        height: 80vh;
    \">
    {}
</div>
        ", formatted_tables)
    }
}

impl HTMLFigureFormatter for Table {
    fn format(&self) -> String {
        let formatted_comment = if self.comment.len() > 0 {
            format!("
<p
    style=\"
        border: dashed 0.5px;
    \">
    {}
</p>
            ", self.comment)
        } else {
            "".to_string()
        };

        format!("
<button
    style=\"
        position: absolute;
        left: calc(80vw * {});
        top: calc(80vh * {});
        width: calc(80vw * {});
        height: calc(80vh * {});
    \"
>
    <div>
        <h1>{}</h1>
        <p>{}</p>
        {}
    </div>
</button>
        ", self.pos.x, self.pos.y, self.pos.width, self.pos.height,
           self.name, self.updated_at, formatted_comment)
    }
}
