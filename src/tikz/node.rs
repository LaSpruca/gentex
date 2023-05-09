use super::{commands::draw::Draw, point::Point, ExtendPath, PathCommand};
#[derive(PartialEq)]
pub enum NodePosition {
    Left,
    Right,
    Above,
    Below,
    AboveLeft,
    AboveRight,
    BelowLeft,
    BelowRight,
    Origin,
}

impl std::fmt::Display for NodePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodePosition::Left => "left",
                NodePosition::Right => "right",
                NodePosition::Above => "above",
                NodePosition::Below => "below",
                NodePosition::AboveLeft => "above left",
                NodePosition::AboveRight => "above right",
                NodePosition::BelowLeft => "below left",
                NodePosition::BelowRight => "below right",
                NodePosition::Origin => "",
            }
        )
    }
}

pub struct Node<T>
where
    T: ExtendPath,
{
    parent: T,
    content: String,
    position: NodePosition,
    midway: bool,
}

impl<T> Node<T>
where
    T: ExtendPath,
{
    fn new(parent: T, content: impl ToString) -> Self {
        Node {
            parent,
            content: content.to_string(),
            position: NodePosition::Origin,
            midway: false,
        }
    }

    pub fn position(mut self, horizontal_position: NodePosition) -> Self {
        self.position = horizontal_position;
        self
    }

    pub fn midway(mut self, midway: bool) -> Self {
        self.midway = midway;
        self
    }

    fn options(&self) -> String {
        let mut options = Vec::new();

        if self.position != NodePosition::Origin {
            options.push(self.position.to_string())
        };
        if self.midway {
            options.push("midway".into());
        }

        if options.is_empty() {
            "".into()
        } else {
            format!(" [{}]", options.join(","))
        }
    }
}

impl<T> ExtendPath for Node<T> where T: ExtendPath {}

impl<T> PathCommand for Node<T>
where
    T: ExtendPath,
{
    fn text(&self) -> String {
        format!(
            "{} node{} {{{}}}",
            self.parent.text(),
            self.options(),
            self.content
        )
    }
}

impl<T> Draw<T>
where
    T: ExtendPath,
{
    pub fn node(self, content: impl ToString) -> Draw<Node<T>> {
        Draw {
            current: Node::new(self.current, content),
        }
    }
}

impl<T> Draw<T>
where
    T: PathCommand,
{
    pub fn node_at(
        self,
        x1: impl num::Num + std::fmt::Display + Copy,
        y1: impl num::Num + std::fmt::Display + Copy,
        content: impl ToString,
    ) -> Draw<Node<Point<T>>> {
        self.point(x1, y1).node(content)
    }
}

impl<T> Draw<Node<T>>
where
    T: ExtendPath,
{
    pub fn position(mut self, horizontal_position: NodePosition) -> Self {
        self.current.position = horizontal_position;
        self
    }

    pub fn midway(mut self, midway: bool) -> Self {
        self.current = self.current.midway(midway);
        self
    }
}

#[cfg(test)]
#[test]
fn does_it_work() {
    use super::TikzPicture;

    let no_options = TikzPicture::begin()
        .and(Draw::new().point(0, 1).node("NO!"))
        .end();

    let vert = TikzPicture::begin()
        .and(
            Draw::new()
                .point(3, 4)
                .node("YES")
                .position(NodePosition::Above)
                .midway(true),
        )
        .end();

    let multiple_options = TikzPicture::begin()
        .and(
            Draw::new()
                .node_at(5, 6, "HELLO!!!")
                .position(NodePosition::Left),
        )
        .end();

    assert_eq!(
        no_options,
        "\\begin{tikzpicture}\n\t\\draw (0, 1) node {NO!};\n\\end{tikzpicture}"
    );
    assert_eq!(
        vert,
        "\\begin{tikzpicture}\n\t\\draw (3, 4) node [above,midway] {YES};\n\\end{tikzpicture}"
    );
    assert_eq!(
        multiple_options,
        "\\begin{tikzpicture}\n\t\\draw (5, 6) node [left] {HELLO!!!};\n\\end{tikzpicture}"
    );
}
