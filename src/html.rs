use crate::data::Tables;

mod figure;
mod table;

pub fn render(tables: &Tables) -> String {
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
    ", figure::render(tables), table::render(tables))
}
