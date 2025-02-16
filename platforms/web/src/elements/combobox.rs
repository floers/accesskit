use accesskit_consumer::Node;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, Event, HtmlElement};

use crate::adapter::{AdapterError, SharedActionHandler};

pub(crate) fn handle_combobox(
    document: &Document,
    node: &Node<'_>,
    action_handler: &SharedActionHandler,
) -> Result<HtmlElement, AdapterError> {
    let e = document
        .create_element("select")?
        .unchecked_into::<HtmlElement>();
    log::debug!("combobox has {} children", node.children().len());
    {
        let target = node.id();
        let action_handler = action_handler.clone();
        let on_change: Closure<dyn Fn(Event)> = Closure::new(move |e: Event| {
            let element: web_sys::HtmlSelectElement = e.target().unwrap().dyn_into().unwrap();
            log::debug!("combobox changed");
            // TODO: handle correct events

            // if let Some(txt) = element.value(). {}
            // if let Some(txt) = e.data() {
            // element.set_value(&txt);
            // action_handler
            //     .borrow_mut()
            //     .do_action(accesskit::ActionRequest {
            //         action: accesskit::Action::SetValue,
            //         target,
            //         data: Some(accesskit::ActionData::Value(txt.into())),
            //     });
            // }
        });
        let on_change = on_change.into_js_value();
        e.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    }
    for child in node.children() {
        let cn = document
            .create_element("option")?
            .unchecked_into::<HtmlElement>();
        if let Some(lbl) = child.label().as_ref() {
            cn.set_attribute("aria-label", lbl)?;
        }
        if let Some(v) = child.value().as_ref() {
            cn.set_attribute("value", v)?;
            cn.set_inner_text(v);
        }
        e.append_child(&cn)?;
    }
    super::shared::delegate_on_click(&e, action_handler.clone(), node.id());
    Ok(e)
}
