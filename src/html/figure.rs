use crate::data::{ Tables, Table };

pub trait HTMLFigureRenderer {
    fn render(&self) -> String;
}

impl HTMLFigureRenderer for Tables {
    fn render(&self) -> String {
        let rendered_tables = self.tables
            .iter()
            .map(|table| table.render())
            .collect::<Vec<String>>()
            .join("\n");

        format!("
<div>
    <form
        method=\"POST\"
        style=\"
            border: solid;
            position: relative;
            width: 90vw;
            height: 90vh;
            margin: auto;
        \">
        {}
    <form>
</div>
        ", rendered_tables)
    }
}

impl HTMLFigureRenderer for Table {
    fn render(&self) -> String {
        let rendered_comment = if self.comment.len() > 0 {
            "
<h2
    style=\"
        position: absolute;
        right: 0;
        top: 0;
    \">
    💬
</h1>
            "
        } else {
            ""
        }.to_string();

        format!("
<button
    type=\"submit\"
    name=\"state\"
    value=\"{}\"
    style=\"
        position: absolute;
        left: calc(90vw * {});
        top: calc(90vh * {});
        width: calc(90vw * {});
        height: calc(90vh * {});
    \"
>
    <div>
        <h1>{}</h1>
        <p>{}</p>
        {}
    </div>
</button>
        ", self.name, self.pos.x, self.pos.y, self.pos.width, self.pos.height,
           self.name, self.updated_at, rendered_comment)
    }
}

pub fn render<T>(elem: &T) -> String
where
    T: HTMLFigureRenderer
{
    elem.render()
}
