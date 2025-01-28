use crate::render::RenderContext;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn render(&mut self, root: impl ViewElement) {}
}

#[derive(Debug, Clone)]
pub struct View {
    ctx: Option<RenderContext>,
    children: Vec<View>,
}

impl View {
    pub fn new() -> Self {
        View {
            ctx: None,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, view: View) {
        self.children.push(view);
    }
    pub fn with(mut self, mut elm: impl ViewElement) -> Self {
        self.ctx = Some(elm.render());
        self
    }
}

pub trait ViewElement: Sized + HasView {
    fn padding(self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self
    }
    fn child(self, child: impl ViewElement) -> Self {
        self
    }
    fn render(&mut self) -> RenderContext;
}

pub trait HasView: Sized {
    fn view(&mut self) -> View {
        View::new()
    }
}

impl<T: HasView> ViewElement for T {
    fn padding(mut self, top: f32, bottom: f32, left: f32, right: f32) -> T {
        self
    }
    fn child(mut self, mut child: impl ViewElement) -> T {
        self.view().add_child(child.view());
        self
    }
    fn render(&mut self) -> RenderContext {
        RenderContext {}
    }
}

pub struct VStack {}
impl ViewElement for VStack {
    fn render(&mut self) -> RenderContext {
        RenderContext {}
    }
}

pub fn vstack() -> impl ViewElement {
    VStack {}
}

#[cfg(test)]
mod test {
    use super::*;

    struct Counter {
        count: i32,
        view: View,
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                count: 0,
                view: View::new(),
            }
        }
        fn render(&mut self) -> impl ViewElement {
            vstack()
        }
    }

    impl HasView for Counter {
        fn view(&mut self) -> View {
            let render = self.render();
            self.view.with(render)
        }
    }

    #[test]
    fn app() {
        App::new().render(Counter::new().padding(10., 10., 10., 10.))
    }
}
