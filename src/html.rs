mod figure;
mod table;

use crate::data::Tables;
use figure::FigureElem;

pub trait HTMLRenderer<T> {
    fn render(&self) -> String;
}

pub struct TopElem;

impl HTMLRenderer<TopElem> for Tables {
    fn render(&self) -> String {
        format!(
            "
<html>
<head>
    <title> Attendance Management</title>
</head>
<body
    style=\"
        margin: 0;
        padding: 0;
    \">
    {}
</body>
</html>
        ",
            <Tables as HTMLRenderer<FigureElem>>::render(self)
        )
    }
}
