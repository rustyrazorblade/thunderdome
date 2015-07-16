use path::Element;

pub struct TreePath {
    pub element: Element,
    pub children: Option<Box<Vec<TreePath>>>
}
