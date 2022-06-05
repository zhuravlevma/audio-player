use crate::infra::next::Next;

pub trait View {
    fn response(command_str: String) -> Next;
}
