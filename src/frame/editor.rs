use super::Frame;

pub struct EditorFrame {}

impl EditorFrame {
    pub fn new() -> Self {
        EditorFrame {}
    }
}

impl Frame for EditorFrame {
    fn tick(&mut self, _ctx: &mut ggez::Context) {
        todo!()
    }

    fn draw(&self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        todo!()
    }

    fn receive_input(
        &mut self,
        _ctx: &mut ggez::Context,
        _input: crate::state::Input,
    ) -> Vec<super::FrameEvent> {
        todo!()
    }
}
