use super::{commands::draw::Draw, ExtendPath, PathCommand};

pub struct Point<T>
where
    T: PathCommand,
{
    parent: T,
    x: String,
    y: String,
}

impl<T> Point<T>
where
    T: PathCommand,
{
    fn new(
        parent: T,
        x: impl num::Num + std::fmt::Display,
        y: impl num::Num + std::fmt::Display,
    ) -> Self {
        Point {
            parent,
            x: x.to_string(),
            y: y.to_string(),
        }
    }
}

impl<T> PathCommand for Point<T>
where
    T: PathCommand,
{
    fn text(&self) -> String {
        format!("{} ({x}, {y})", self.parent.text(), x = self.x, y = self.y)
    }
}

impl<T> ExtendPath for Point<T> where T: PathCommand {}

impl<T> Draw<T>
where
    T: PathCommand,
{
    pub fn point(
        self,
        x: impl num::Num + std::fmt::Display,
        y: impl num::Num + std::fmt::Display,
    ) -> Draw<Point<T>> {
        Draw {
            current: Point::new(self.current, x, y),
        }
    }
}

#[cfg(test)]
#[test]
fn does_it_work() {
    use crate::tikz::TikzPicture;

    let tikz = TikzPicture::begin().and(Draw::new().point(1, 2)).end();
    assert_eq!(
        "\\begin{tikzpicture}\n\t\\draw (1, 2);\n\\end{tikzpicture}",
        tikz
    );
}
