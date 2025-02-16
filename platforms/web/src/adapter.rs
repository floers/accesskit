// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActionHandler, ActivationHandler, NodeId, TreeUpdate};
use accesskit_consumer::{FilterResult, Node, Tree, TreeChangeHandler};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{ClipboardEvent, Document, Element, Event, HtmlElement, InputEvent, KeyboardEvent};

use crate::{elements, filters::filter, node::NodeWrapper};

#[derive(Debug)]
pub(crate) struct AdapterError {
    msg: String,
}
impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.msg)
    }
}
impl From<JsValue> for AdapterError {
    fn from(value: JsValue) -> Self {
        Self {
            msg: value
                .as_string()
                .unwrap_or_else(|| "Unknown error".to_string()),
        }
    }
}

enum State {
    Pending {
        is_host_focused: bool,
        document: Document,
        parent: Element,
    },
    Active {
        tree: Tree,
        document: Document,
        root: HtmlElement,
        elements: HashMap<NodeId, HtmlElement>,
    },
}

pub(crate) type SharedActionHandler = Rc<RefCell<dyn ActionHandler>>;

pub struct Adapter {
    state: State,
    action_handler: SharedActionHandler,
}

impl Adapter {
    pub fn new(
        parent_id: &str,
        mut activation_handler: impl ActivationHandler,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Self {
        let action_handler = Rc::new(RefCell::new(action_handler));
        // TODO remove unwraps
        let document = web_sys::window().unwrap().document().unwrap();
        let parent = document.get_element_by_id(parent_id).unwrap();

        let state = match activation_handler.request_initial_tree() {
            Some(initial_state) => {
                let tree = Tree::new(initial_state, true);
                let (root, elements) =
                    add_initial_tree(&document, &parent, &tree, action_handler.clone());
                State::Active {
                    tree,
                    document,
                    root,
                    elements,
                }
            }
            None => State::Pending {
                is_host_focused: true,
                document,
                parent,
            },
        };
        Self {
            state,
            action_handler,
        }
    }

    pub fn update_if_active(&mut self, update_factory: impl FnOnce() -> TreeUpdate) {
        match &mut self.state {
            State::Pending {
                is_host_focused,
                document,
                parent,
            } => {
                let tree = Tree::new(update_factory(), *is_host_focused);
                let (root, elements) =
                    add_initial_tree(document, parent, &tree, self.action_handler.clone());
                self.state = State::Active {
                    tree,
                    document: document.clone(),
                    root,
                    elements,
                };
            }
            State::Active {
                tree,
                document,
                elements,
                ..
            } => {
                let mut handler = AdapterChangeHandler {
                    document,
                    elements,
                    action_handler: self.action_handler.clone(),
                };
                tree.update_and_process_changes(update_factory(), &mut handler);
            }
        }
    }

    pub fn update_host_focus_state(&mut self, is_focused: bool) {
        match &mut self.state {
            State::Pending {
                is_host_focused, ..
            } => *is_host_focused = is_focused,
            State::Active {
                tree,
                document,
                elements,
                ..
            } => {
                let mut handler = AdapterChangeHandler {
                    document,
                    elements,
                    action_handler: self.action_handler.clone(),
                };
                tree.update_host_focus_state_and_process_changes(is_focused, &mut handler);
            }
        }
    }
}

fn add_initial_tree(
    document: &Document,
    parent: &Element,
    tree: &Tree,
    action_handler: SharedActionHandler,
) -> (HtmlElement, HashMap<NodeId, HtmlElement>) {
    let root = document
        .create_element("div")
        .unwrap()
        .unchecked_into::<HtmlElement>();
    root.set_attribute("role", "application").unwrap();
    parent.append_child(&root).unwrap();
    let mut elements = HashMap::new();
    let root_node = tree.state().root();
    add_element_recursive(document, &root, &root_node, &mut elements, action_handler);
    if let Some(focus_id) = tree.state().focus_id() {
        if let Some(element) = elements.get(&focus_id) {
            focus(element);
        }
    }
    (root, elements)
}

fn add_element(
    document: &Document,
    parent: &HtmlElement,
    node: &Node,
    elements: &mut HashMap<NodeId, HtmlElement>,
    action_handler: SharedActionHandler,
) -> Result<HtmlElement, AdapterError> {
    let element = match node.role() {
        accesskit::Role::Button => elements::handle_button(document, node, &action_handler)?,
        accesskit::Role::TextInput => elements::handle_input(document, node, &action_handler)?,
        accesskit::Role::CheckBox => elements::handle_checkbox(document, node, &action_handler)?,
        accesskit::Role::ComboBox => elements::handle_combobox(document, node, &action_handler)?,
        _ => document
            .create_element("div")?
            .unchecked_into::<HtmlElement>(),
    };

    if let Some(bb) = node.bounding_box() {
        let style = element.style();
        let _ = style.set_property("position", "absolute");
        let _ = style.set_property("top", &format!("{}px", bb.min_y()));
        let _ = style.set_property("left", &format!("{}px", bb.min_x()));
        let _ = style.set_property("width", &format!("{}px", bb.width()));
        let _ = style.set_property("height", &format!("{}px", bb.height()));
    }
    let wrapper = NodeWrapper(*node);
    wrapper.set_all_attributes(&element);
    parent.append_child(&element)?;
    elements.insert(node.id(), element.clone());
    Ok(element)
}

fn add_element_recursive(
    document: &Document,
    parent: &HtmlElement,
    node: &Node,
    elements: &mut HashMap<NodeId, HtmlElement>,
    action_handler: SharedActionHandler,
) {
    if let Ok(element) = add_element(document, parent, node, elements, action_handler.clone()) {
        for child in node.filtered_children(&filter) {
            add_element_recursive(document, &element, &child, elements, action_handler.clone());
        }
    }
}

fn focus(element: &HtmlElement) {
    element.focus().unwrap();
}

fn blur(element: &HtmlElement) {
    element.blur().unwrap();
}

struct AdapterChangeHandler<'a> {
    document: &'a Document,
    elements: &'a mut HashMap<NodeId, HtmlElement>,
    action_handler: SharedActionHandler,
}

impl TreeChangeHandler for AdapterChangeHandler<'_> {
    fn node_added(&mut self, node: &Node) {
        if filter(node) != FilterResult::Include {
            return;
        }
        if self.elements.contains_key(&node.id()) {
            return;
        }
        if let Some(parent) = node.filtered_parent(&filter) {
            if let Some(parent_element) = self.elements.get(&parent.id()).cloned() {
                let _ = add_element(
                    self.document,
                    &parent_element,
                    node,
                    self.elements,
                    self.action_handler.clone(),
                );
            }
        }
    }

    fn node_updated(&mut self, old_node: &Node, new_node: &Node) {
        if filter(new_node) != FilterResult::Include {
            return;
        }
        let element = match self.elements.get(&new_node.id()) {
            Some(element) => element,
            None => {
                return;
            }
        };
        let old_wrapper = NodeWrapper(*old_node);
        let new_wrapper = NodeWrapper(*new_node);
        new_wrapper.update_attributes(element, &old_wrapper);
    }

    fn focus_moved(&mut self, old_node: Option<&Node>, new_node: Option<&Node>) {
        if let Some(new_node) = new_node {
            if let Some(element) = self.elements.get(&new_node.id()) {
                focus(element);
            }
        } else if let Some(old_node) = old_node {
            if let Some(element) = self.elements.get(&old_node.id()) {
                blur(element);
            }
        }
    }

    fn node_removed(&mut self, node: &Node) {
        if let Some(element) = self.elements.remove(&node.id()) {
            element.remove();
        }
    }
}
