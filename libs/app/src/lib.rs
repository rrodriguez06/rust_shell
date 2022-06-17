pub enum InputMode {
    Normal,
    Editing,
    Completion,
    History,
    Helper,
    Output,
}

pub struct App {
    pub input: String,
    pub command: String,
    pub input_mode: InputMode,
    pub output: String,
    pub path: String,
    pub completion: Vec<String>,
    pub completion_display: Vec<String>,
    pub completion_index: usize,
    pub history: Vec<String>,
    pub history_index: usize,
    pub helper: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            command: String::new(),
            input_mode: InputMode::Normal,
            output: String::new(),
            path: std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            completion: Vec::new(),
            completion_display: Vec::new(),
            completion_index: 0,
            history: Vec::new(),
            history_index: 0,
            helper: vec![
                String::from("<h1>Keybindings:</h1>"),
                String::from("    <h2>NORMAL MODE</h2>"),
                String::from("        <c>I:</c>      <i>enter insert mode</i>"),
                String::from("        <c>O:</c>      <i>quit shell</i>"),
                String::from("    <h2>INSERT MODE</h2>"),
                String::from("        <c>Tab:</c>    <i>enable completion mode</i>"),
                String::from("        <c>Down:</c>   <i>enable history mode</i>"),
                String::from("    <h2>COMPLETION MODE</h2>"),
                String::from("        <c>Tab:</c>    <i>select completion pattern</i>"),
                String::from("        <c>Enter:</c>  <i>use selected completion pattern</i>"),
                String::from("        <c>Esc:</c>    <i>exit completion mode</i>"),
                String::from("    <h2>HISTORY MODE</h2>"),
                String::from("        <c>Tab:</c>    <i>select history command</i>"),
                String::from("        <c>Enter:</c>  <i>use selected history command</i>"),
                String::from("        <c>Esc:</c>    <i>exit history mode</i>"),
                String::from("<h1>Custom commands:</h1>"),
                String::from("    <c>help:</c>    <i>show helping popup</i>"),
                String::from("    <c>c:</c>       <i>clear output</i>"),
            ]
        }
    }
}
