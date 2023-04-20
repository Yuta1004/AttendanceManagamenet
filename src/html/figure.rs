use crate::data::{Table, TableStat, Tables};
use crate::html::HTMLRenderer;

pub(super) struct FigureElem;

impl HTMLRenderer<FigureElem> for Tables {
    fn render(&self) -> String {
        let rendered_tables = self
            .tables
            .iter()
            .map(<Table as HTMLRenderer<FigureElem>>::render)
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "
<div
    style=\"
        position: relative;
        width: 100vw;
        height: 100vh;
        margin: auto;
    \">
    <form method=\"POST\">{}</form>
    <p
        style=\"
            position: absolute;
            right: 0;
            bottom: 0;
        \">
        最終更新 : {}
    </p>
</div>
        ",
            rendered_tables, self.updated_at
        )
    }
}

impl HTMLRenderer<FigureElem> for Table {
    fn render(&self) -> String {
        let button_color = match self.state {
            TableStat::Occupied => "lime",
            TableStat::Vacant => "pink",
        };

        format!(
            "
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
    </div>
</button>
        ",
            self.name,
            self.pos.x,
            self.pos.y,
            self.pos.width,
            self.pos.height,
            button_color,
            self.name,
            self.updated_at
        )
    }
}
