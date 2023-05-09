use super::{commands::draw::Draw, point::Point, ExtendPath, PathCommand};

pub struct Rectangle<T>
where
    T: ExtendPath,
{
    parent: T,
    x: String,
    y: String,
    thick: bool,
}

impl<T> Rectangle<T>
where
    T: ExtendPath,
{
    fn new(
        parent: T,
        x: impl num::Num + std::fmt::Display,
        y: impl num::Num + std::fmt::Display,
    ) -> Self {
        Rectangle {
            parent,
            x: x.to_string(),
            y: y.to_string(),
            thick: false,
        }
    }
    pub fn thick(mut self, thick: bool) -> Self {
        self.thick = thick;
        self
    }

    fn options(&self) -> String {
        let mut options = Vec::new();
        if self.thick {
            options.push("thick".to_string());
        }

        if options.is_empty() {
            "".into()
        } else {
            format!(" [{}]", options.join(","))
        }
    }
}

impl<T> ExtendPath for Rectangle<T> where T: ExtendPath {}

impl<T> PathCommand for Rectangle<T>
where
    T: ExtendPath,
{
    fn text(&self) -> String {
        format!(
            "{}{} rectangle ({x}, {y})",
            self.parent.text(),
            self.options(),
            x = self.x,
            y = self.y
        )
    }
}

impl<T> Draw<T>
where
    T: ExtendPath,
{
    pub fn extend_rectangle(
        self,
        x: impl num::Num + std::fmt::Display,
        y: impl num::Num + std::fmt::Display,
    ) -> Draw<Rectangle<T>> {
        Draw {
            current: Rectangle::new(self.current, x, y),
        }
    }
}

impl<T> Draw<T>
where
    T: PathCommand,
{
    pub fn rectangle(
        self,
        x1: impl num::Num + std::fmt::Display,
        y1: impl num::Num + std::fmt::Display,
        x2: impl num::Num + std::fmt::Display,
        y2: impl num::Num + std::fmt::Display,
    ) -> Draw<Rectangle<Point<T>>> {
        self.point(x1, y1).extend_rectangle(x2, y2)
    }
}

impl<T> Draw<Rectangle<T>>
where
    T: ExtendPath,
{
    pub fn thick(mut self, thick: bool) -> Self {
        self.current = self.current.thick(thick);
        self
    }
}

#[cfg(test)]
#[test]
fn does_it_work() {
    use super::TikzPicture;

    let no_options = TikzPicture::begin()
        .and(Draw::new().point(0, 0).extend_rectangle(1, 1))
        .end();

    let thick = TikzPicture::begin()
        .and(Draw::new().rectangle(0, 0, 1, 1).thick(true))
        .end();

    assert_eq!(
        no_options,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) rectangle (1, 1);\n\\end{tikzpicture}"
    );
    assert_eq!(
        thick,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) [thick] rectangle (1, 1);\n\\end{tikzpicture}"
    );
}
