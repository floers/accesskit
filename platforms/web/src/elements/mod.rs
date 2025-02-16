mod button;
pub use button::*;

mod input;
pub use input::*;

mod combobox;
pub use combobox::*;

mod checkbox;
pub use checkbox::*;

mod shared {
    use accesskit::{ActionHandler, NodeId};
    use std::{cell::RefCell, rc::Rc};
    use wasm_bindgen::{prelude::Closure, JsCast};
    use web_sys::HtmlElement;

    pub(crate) fn delegate_on_click(
        e: &HtmlElement,
        action_handler: Rc<RefCell<dyn ActionHandler>>,
        target: NodeId,
    ) {
        let dbg = e.tag_name();
        let cb: Closure<dyn Fn()> = Closure::new(move || {
            log::debug!("clicked on {dbg}");
            action_handler
                .borrow_mut()
                .do_action(accesskit::ActionRequest {
                    action: accesskit::Action::Click,
                    target,
                    data: None,
                });
        });
        let cb = cb.into_js_value();
        e.set_onclick(Some(cb.as_ref().unchecked_ref()));
    }
}
