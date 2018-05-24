use control::ControlObject;

pub trait View {
    fn create_view(&mut self) -> Box<ControlObject>;
}
