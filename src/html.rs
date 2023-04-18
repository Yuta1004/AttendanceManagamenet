use crate::data::{ Tables };

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
    作成中
    <h2>座席情報</h2>
    {}
</body>
</html>
    ", gen_list(tables))
}

fn gen_list<T>(elem: &T) -> String
where
    T: list::HTMLListFormatter
{
    elem.format()
}
