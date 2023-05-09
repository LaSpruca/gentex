use crate::tikz::{Command, PathCommand};

pub struct Draw<T>
where
    T: PathCommand,
{
    pub(crate) current: T,
}

impl Draw<Noop> {
    pub fn new() -> Self {
        Draw { current: Noop }
    }
}

impl Default for Draw<Noop> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Command for Draw<T>
where
    T: PathCommand,
{
    fn text(&self) -> String {
        format!("\\draw{};", self.current.text())
    }
}

pub struct Noop;

impl PathCommand for Noop {
    fn text(&self) -> String {
        "".into()
    }
}
