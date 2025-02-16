use accesskit_consumer::Node;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{ClipboardEvent, Document, Event, HtmlElement, InputEvent, KeyboardEvent};

use crate::adapter::{AdapterError, SharedActionHandler};

pub(crate) fn handle_input(
    document: &Document,
    node: &Node<'_>,
    action_handler: &SharedActionHandler,
) -> Result<HtmlElement, AdapterError> {
    let e = document
        .create_element("input")?
        .unchecked_into::<HtmlElement>();
    let target = node.id();
    {
        let action_handler = action_handler.clone();
        let on_keypress: Closure<dyn Fn(KeyboardEvent)> = Closure::new(move |e: KeyboardEvent| {
            let input_element: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let key = e.key();
            let txt = format!("{}{key}", input_element.value());
            input_element.set_value(&txt);
            action_handler
                .borrow_mut()
                .do_action(accesskit::ActionRequest {
                    action: accesskit::Action::SetValue,
                    target,
                    data: Some(accesskit::ActionData::Value(txt.into())),
                });
        });
        let on_keypress = on_keypress.into_js_value();
        e.set_onkeypress(Some(on_keypress.as_ref().unchecked_ref()));
    }
    {
        let action_handler = action_handler.clone();
        let on_change: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
            let input_element: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let txt = input_element.value();
            input_element.set_value(&txt);
            action_handler
                .borrow_mut()
                .do_action(accesskit::ActionRequest {
                    action: accesskit::Action::SetValue,
                    target,
                    data: Some(accesskit::ActionData::Value(txt.into())),
                });
        });
        let on_change = on_change.into_js_value();
        e.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    }
    {
        let action_handler = action_handler.clone();
        let on_paste: Closure<dyn Fn(ClipboardEvent)> = Closure::new(move |e: ClipboardEvent| {
            let input_element: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            if let Some(txt) = e
                .clipboard_data()
                .and_then(|d| d.get_data("text/plain").ok())
            {
                input_element.set_value(&txt);
                action_handler
                    .borrow_mut()
                    .do_action(accesskit::ActionRequest {
                        action: accesskit::Action::SetValue,
                        target,
                        data: Some(accesskit::ActionData::Value(txt.into())),
                    });
            }
        });
        let on_paste = on_paste.into_js_value();
        e.set_onpaste(Some(on_paste.as_ref().unchecked_ref()));
    }
    {
        let action_handler = action_handler.clone();
        let on_input: Closure<dyn Fn(InputEvent)> = Closure::new(move |e: InputEvent| {
            let input_element: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            if let Some(txt) = e.data() {
                input_element.set_value(&txt);
                action_handler
                    .borrow_mut()
                    .do_action(accesskit::ActionRequest {
                        action: accesskit::Action::SetValue,
                        target,
                        data: Some(accesskit::ActionData::Value(txt.into())),
                    });
            }
        });
        let on_input = on_input.into_js_value();
        e.set_oninput(Some(on_input.as_ref().unchecked_ref()));
    }
    super::shared::delegate_on_click(&e, action_handler.clone(), node.id());
    Ok(e)
}
