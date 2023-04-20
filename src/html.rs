use crate::data::Tables;

mod figure;
mod table;

pub fn render(tables: &Tables) -> String {
    format!("
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
    ", figure::render(tables))
}
