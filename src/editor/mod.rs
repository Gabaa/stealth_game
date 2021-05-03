pub struct SelectionHandler {
    pub dragged_actor: Option<usize>,
}

impl SelectionHandler {
    pub fn new() -> Self {
        SelectionHandler {
            dragged_actor: None,
        }
    }
}
