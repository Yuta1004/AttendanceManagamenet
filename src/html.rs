use crate::data::{ Tables };

mod figure;
mod list;

pub fn format(tables: &Tables) -> String {
    format!("
<html>
<head>
    <title> Attendance Management</title>
</head>
<body>
    <h1>出席情報</h1>
    <h2>座席表</h2>
    {}
    <h2>座席情報</h2>
    {}
</body>
</html>
    ", gen_figure(tables), gen_list(tables))
}

fn gen_list<T>(elem: &T) -> String
where
    T: list::HTMLListFormatter
{
    elem.format()
}

fn gen_figure<T>(elem: &T) -> String
where
    T: figure::HTMLFigureFormatter
{
    elem.format()
}
