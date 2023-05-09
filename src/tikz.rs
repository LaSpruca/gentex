pub mod circle;
pub mod commands;
pub mod line;
pub mod node;
pub mod point;
pub mod rectangle;

pub struct TikzPicture {
    commands: Vec<Box<dyn Command>>,
}

impl TikzPicture {
    pub fn begin() -> Self {
        TikzPicture {
            commands: Vec::new(),
        }
    }

    pub fn and<U>(mut self, command: U) -> TikzPicture
    where
        U: Command + 'static,
    {
        self.commands.push(Box::new(command));
        self
    }

    pub fn push_command<U>(&mut self, command: U)
    where
        U: Command + 'static,
    {
        self.commands.push(Box::new(command));
    }

    pub fn end(self) -> String {
        format!(
            "\\begin{{tikzpicture}}\n\t{}\n\\end{{tikzpicture}}",
            self.commands
                .iter()
                .map(|f| f.text())
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}

pub trait Command {
    fn text(&self) -> String;
}

pub trait PathCommand {
    fn text(&self) -> String;
}
pub trait ExtendPath: PathCommand {}

pub enum ArrowType {
    Default,
    Stealth,
}

impl std::fmt::Display for ArrowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArrowType::Default => "",
                ArrowType::Stealth => "stealth",
            }
        )
    }
}
