use crate::app::View;
use crate::app::{HasView, ViewElement};

pub struct Container {}

pub struct Vstack {}

impl HasView for Vstack {
    fn view(self) -> View {
        View::new()
    }
}

pub fn vstack() -> impl ViewElement {
    Vstack {}
}
