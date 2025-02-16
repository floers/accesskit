use accesskit_consumer::Node;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::adapter::{AdapterError, SharedActionHandler};

pub(crate) fn handle_button(document: &Document, node: &Node<'_>, action_handler: &SharedActionHandler) -> Result<HtmlElement, AdapterError> {
    let e = document
        .create_element("button")?
        .unchecked_into::<HtmlElement>();
    super::shared::delegate_on_click(&e, action_handler.clone(), node.id());
    Ok(e)
}