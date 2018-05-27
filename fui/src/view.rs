use control::ControlObject;

pub trait View {
    fn create_view(&self) -> Box<ControlObject>;
}
