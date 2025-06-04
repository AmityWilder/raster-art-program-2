pub mod art;
pub mod color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Editor {
    Art,
    Color,
}
