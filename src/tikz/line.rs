use super::{commands::draw::Draw, point::Point, ArrowType, ExtendPath, PathCommand};

pub struct Line<T>
where
    T: ExtendPath,
{
    parent: T,
    x: String,
    y: String,
    thick: bool,
    start: ArrowType,
    end: ArrowType,
}

impl<T> Line<T>
where
    T: ExtendPath,
{
    fn new(
        parent: T,
        x: impl num::Num + std::fmt::Display,
        y: impl num::Num + std::fmt::Display,
    ) -> Self {
        Line {
            parent,
            x: x.to_string(),
            y: y.to_string(),
            thick: false,
            start: ArrowType::Default,
            end: ArrowType::Default,
        }
    }

    pub fn start(mut self, start: ArrowType) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: ArrowType) -> Self {
        self.end = end;
        self
    }

    pub fn thick(mut self, thick: bool) -> Self {
        self.thick = thick;
        self
    }

    fn options(&self) -> String {
        let mut options = Vec::new();
        if self.thick {
            options.push("thick".into());
        }

        match (&self.start, &self.end) {
            (ArrowType::Default, ArrowType::Default) => {}
            (start, end) => options.push(format!("{}-{}", start, end)),
        };

        if options.is_empty() {
            "".into()
        } else {
            format!(" [{}]", options.join(","))
        }
    }
}

impl<T> ExtendPath for Line<T> where T: ExtendPath {}

impl<T> PathCommand for Line<T>
where
    T: ExtendPath,
{
    fn text(&self) -> String {
        format!(
            "{}{} -- ({}, {})",
            self.parent.text(),
            self.options(),
            self.x,
            self.y
        )
    }
}

impl<T> Draw<T>
where
    T: ExtendPath,
{
    pub fn extend_line(
        self,
        x: impl num::Num + std::fmt::Display,
        y: impl num::Num + std::fmt::Display,
    ) -> Draw<Line<T>> {
        Draw {
            current: Line::new(self.current, x, y),
        }
    }
}

impl<T> Draw<T>
where
    T: PathCommand,
{
    pub fn line(
        self,
        x1: impl num::Num + std::fmt::Display,
        y1: impl num::Num + std::fmt::Display,
        x2: impl num::Num + std::fmt::Display,
        y2: impl num::Num + std::fmt::Display,
    ) -> Draw<Line<Point<T>>> {
        self.point(x1, y1).extend_line(x2, y2)
    }
}

impl<T> Draw<Line<T>>
where
    T: ExtendPath,
{
    pub fn arrow_start(mut self, style: ArrowType) -> Self {
        self.current = self.current.start(style);
        self
    }

    pub fn arrow_end(mut self, style: ArrowType) -> Self {
        self.current = self.current.end(style);
        self
    }

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
        .and(Draw::new().point(0, 0).extend_line(1, 1))
        .end();

    let thick = TikzPicture::begin()
        .and(Draw::new().point(0, 0).extend_line(1, 1).thick(true))
        .end();

    let styled_arrow = TikzPicture::begin()
        .and(
            Draw::new()
                .point(0, 0)
                .extend_line(1, 1)
                .arrow_start(ArrowType::Stealth),
        )
        .end();

    let multiple_options = TikzPicture::begin()
        .and(
            Draw::new()
                .line(0, 0, 1, 1)
                .arrow_start(ArrowType::Stealth)
                .arrow_end(ArrowType::Stealth)
                .thick(true),
        )
        .end();

    assert_eq!(
        no_options,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) -- (1, 1);\n\\end{tikzpicture}"
    );
    assert_eq!(
        thick,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) [thick] -- (1, 1);\n\\end{tikzpicture}"
    );
    assert_eq!(
        styled_arrow,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) [stealth-] -- (1, 1);\n\\end{tikzpicture}"
    );
    assert_eq!(
        multiple_options,
        "\\begin{tikzpicture}\n\t\\draw (0, 0) [thick,stealth-stealth] -- (1, 1);\n\\end{tikzpicture}");
}
