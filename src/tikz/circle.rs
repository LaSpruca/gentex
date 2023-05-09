use super::{commands::draw::Draw, point::Point, ExtendPath, PathCommand};

pub struct Circle<T>
where
    T: ExtendPath,
{
    parent: T,
    radius: String,
    thick: bool,
}

impl<T> Circle<T>
where
    T: ExtendPath,
{
    fn new(parent: T, radius: impl num::Num + std::fmt::Display) -> Self {
        Circle {
            parent,
            radius: radius.to_string(),
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

impl<T> ExtendPath for Circle<T> where T: ExtendPath {}

impl<T> PathCommand for Circle<T>
where
    T: ExtendPath,
{
    fn text(&self) -> String {
        format!(
            "{}{} circle ({})",
            self.parent.text(),
            self.options(),
            self.radius
        )
    }
}

impl<T> Draw<T>
where
    T: ExtendPath,
{
    pub fn extend_circle(self, radius: impl num::Num + std::fmt::Display) -> Draw<Circle<T>> {
        Draw {
            current: Circle::new(self.current, radius),
        }
    }
}

impl<T> Draw<T>
where
    T: PathCommand,
{
    pub fn circle(
        self,
        x1: impl num::Num + std::fmt::Display,
        y1: impl num::Num + std::fmt::Display,
        radius: impl num::Num + std::fmt::Display,
    ) -> Draw<Circle<Point<T>>> {
        self.point(x1, y1).extend_circle(radius)
    }
}

impl<T> Draw<Circle<T>>
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
        .and(Draw::new().point(0, 0).extend_circle(1))
        .end();

    let thick = TikzPicture::begin()
        .and(Draw::new().circle(0, 0, 1).thick(true))
        .end();

    assert_eq!(
        no_options,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) circle (1);\n\\end{tikzpicture}"
    );
    assert_eq!(
        thick,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) [thick] circle (1);\n\\end{tikzpicture}"
    );
}
