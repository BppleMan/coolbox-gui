use crate::tasks::ExecutableState;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum CoolState {
    Ready,
    Installing,
    Installed,
}

pub trait StateAble {
    fn current_state(&mut self) -> &mut ExecutableState;

    fn outputs(&mut self) -> &mut Vec<String>;

    fn errors(&mut self) -> &mut Vec<String>;
}
