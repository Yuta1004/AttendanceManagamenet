use crate::data::{ Tables, Table, TableStat };

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
            position: relative;
            width: 100vw;
            height: 100vh;
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
        let button_color = match self.state {
            TableStat::Occupied => "lime",
            TableStat::Vacant => "pink"
        };

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
        left: calc(100vw * {});
        top: calc(100vh * {});
        width: calc(100vw * {});
        height: calc(100vh * {});
        background-color: {};
    \"
>
    <div>
        <h1>{}</h1>
        <p>{}</p>
        {}
    </div>
</button>
        ", self.name, self.pos.x, self.pos.y, self.pos.width, self.pos.height,
           button_color, self.name, self.updated_at, rendered_comment)
    }
}

pub fn render<T>(elem: &T) -> String
where
    T: HTMLFigureRenderer
{
    elem.render()
}
