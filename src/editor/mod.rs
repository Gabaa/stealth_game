pub enum DraggableObject {
    Actor { index: usize },
    EndArea,
}

pub struct SelectionHandler {
    pub dragged_object: Option<DraggableObject>,
}

impl SelectionHandler {
    pub fn new() -> Self {
        SelectionHandler {
            dragged_object: None,
        }
    }
}
