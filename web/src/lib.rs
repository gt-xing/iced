use dodrio::bumpalo;
use futures::Future;
use std::cell::RefCell;

mod bus;
mod color;
mod element;
mod widget;

pub use bus::Bus;
pub use color::Color;
pub use element::Element;
pub use iced::Align;
pub use widget::*;

pub trait UserInterface {
    type Message;

    fn update(
        &mut self,
        message: Self::Message,
    ) -> Option<Box<dyn Future<Output = Self::Message>>>;

    fn view(&mut self) -> Element<Self::Message>;

    fn run(self)
    where
        Self: 'static + Sized,
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let app = Application::new(self);

        let vdom = dodrio::Vdom::new(&body, app);
        vdom.forget();
    }
}

struct Application<Message> {
    ui: RefCell<Box<dyn UserInterface<Message = Message>>>,
}

impl<Message> Application<Message> {
    fn new(ui: impl UserInterface<Message = Message> + 'static) -> Self {
        Self {
            ui: RefCell::new(Box::new(ui)),
        }
    }

    fn update(&mut self, message: Message) {
        let mut ui = self.ui.borrow_mut();

        // TODO: Resolve futures and publish resulting messages
        let _ = ui.update(message);
    }
}

impl<Message> dodrio::Render for Application<Message>
where
    Message: 'static,
{
    fn render<'a, 'bump>(
        &'a self,
        bump: &'bump bumpalo::Bump,
    ) -> dodrio::Node<'bump>
    where
        'a: 'bump,
    {
        let mut ui = self.ui.borrow_mut();
        let element = ui.view();

        element.widget.node(bump, &Bus::new())
    }
}
