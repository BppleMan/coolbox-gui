#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum CoolState {
    Ready,
    Installing,
    Installed,
}
