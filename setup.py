import os


for day in range(1, 13):
    file_name = "day_{:02d}.rs".format(day)
    content = """use crate::TaskCompleter;
    
pub struct Task{0};

impl TaskCompleter for Task{0} {{
    fn do_task_1(&self) -> String {{
        "".to_string()
    }}

    fn do_task_2(&self) -> String {{
        "".to_string()
    }}

    fn task_1_result(&self) -> Option<String> {{
        None
    }}

    fn task_2_result(&self) -> Option<String> {{
        None
    }}
}}
""".format(
        day
    )
    with open(".\\src\\" + file_name, "w") as f:
        f.write(content)
    os.mkdir(".\\input\\day_{:02d}".format(day))
