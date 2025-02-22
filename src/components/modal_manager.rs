use std::collections::HashMap;

use dioxus::prelude::*;
use js_sys::*;
use uuid::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::init::MODAL_MANAGER;

// extern "C" {
//     pub type ResizeObserver;
// }

// 定义弹窗类型和级别
#[derive(Clone, PartialEq, Debug)]
pub enum ModalType {
    Modal,
    Dialog,
    Message,
    Popover,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Modal {
    modal_type: ModalType,
    content: Element,
    is_open: bool,
    level: u8,
    position: Option<(f64, f64)>,
}

impl Modal {
    pub fn change_visible(&mut self) {
        self.is_open = !self.is_open;
    }
    pub fn open(&mut self) {
        self.is_open = true;
    }
    pub fn close(&mut self) {
        self.is_open = false;
    }
}
#[derive(Clone, Debug)]
pub struct ModalManager {
    modals: HashMap<String, Modal>,
    levels: HashMap<u8, Vec<String>>,
}

impl ModalManager {
    // 初始化弹窗管理器
    pub fn new() -> Self {
        Self {
            modals: HashMap::new(),
            levels: HashMap::new(),
        }
    }

    // 添加Modal
    pub fn add_modal(&mut self, content: Element, id: String) -> String {
        self.add_custom_id_modal(ModalType::Modal, content, 1, id)
    }

    // 添加Dialog
    pub fn add_dialog(&mut self, content: Element) -> String {
        self.add_generic_modal(ModalType::Dialog, content, 2, None)
    }

    // 添加Message
    pub fn add_message(&mut self, content: Element, id: String) -> String {
        self.add_custom_id_modal(ModalType::Modal, content, 3, id)
    }

    // 添加Popover
    pub fn add_popover(&mut self, content: Element, position: (f64, f64)) -> String {
        self.add_generic_modal(ModalType::Popover, content, 4, Some(position))
    }
    pub fn update_popover_position(&mut self, id: &str, position: (f64, f64)) {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.position = Some(position);
            let ele = window().unwrap().document().unwrap().get_element_by_id(id);
            if let Some(ele) = ele {
                ele.set_attribute(
                    "style",
                    &format!(
                        "position:absolute; left: {}px; top: {}px;",
                        position.0, position.1
                    ),
                )
                .unwrap();
            }
        }
    }
    pub fn has_modal(&self, id: &str) -> bool {
        self.modals.contains_key(id)
    }
    pub fn change_visible(&mut self, id: &str) {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.change_visible();
        }
    }
    // 通用的添加弹窗方法
    fn add_generic_modal(
        &mut self,
        modal_type: ModalType,
        content: Element,
        level: u8,
        position: Option<(f64, f64)>,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let modal = Modal {
            modal_type,
            content,
            is_open: false,
            level,
            position,
        };
        self.modals.insert(id.clone(), modal);

        // manager levels
        self.levels.entry(level).or_default().push(id.clone());

        id
    }

    // 通用的添加弹窗方法
    fn add_custom_id_modal(
        &mut self,
        modal_type: ModalType,
        content: Element,
        level: u8,
        id: String,
    ) -> String {
        let modal = Modal {
            modal_type,
            content,
            is_open: false,
            level,
            position: None,
        };
        self.modals.insert(id.clone(), modal);

        // manager levels
        self.levels.entry(level).or_default().push(id.clone());

        id
    }

    // open modal
    pub fn open_modal(&mut self, current_id: &str) {
        for (id, modal) in self.modals.iter_mut() {
            if id != current_id {
                modal.close();
            } else {
                modal.open();
            }
        }
    }

    // close modal
    pub fn close_modal(&mut self, id: &str) {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.close();
        }
    }

    // destroy modal
    pub fn destroy_modal(&mut self, id: &str) {
        if self.has_modal(id) {
            if let Some(modal) = self.modals.remove(id) {
                if let Some(level_modals) = self.levels.get_mut(&modal.level) {
                    level_modals.retain(|modal_id| modal_id != id);
                }
            }
        }
    }

    // 根据层级关闭所有弹窗
    pub fn destory_all_modals_by_level(&mut self, level: u8) {
        if let Some(level_modals) = self.levels.get(&level) {
            let modals_to_close = level_modals.clone();
            for id in modals_to_close {
                self.destroy_modal(&id);
            }
        }
    }

    // 关闭所有弹窗
    pub fn destory_all_modals(&mut self) {
        let levels_to_close = self.levels.keys().cloned().collect::<Vec<u8>>();
        for level in levels_to_close {
            self.destory_all_modals_by_level(level);
        }
    }
}

#[component]
fn ModalComponent(modal: Modal, id: String) -> Element {
    let style = match modal.modal_type {
        ModalType::Popover => {
            if let Some((x, y)) = modal.position {
                format!(
                    "left: {}px; top: {}px; position: absolute; z-index: {};",
                    x,
                    y,
                    modal.level + 100
                )
            } else {
                format!("z-index: {};", modal.level + 100)
            }
        }
        _ => format!("z-index: {};", modal.level + 100),
    };
    rsx! {
        div {
            id: id,
            style: "position:fixed; {style}",
            div { class: "modal-content", {modal.content.clone()} }
        }
    }
}

#[component]
pub fn ModalManagerProvider() -> Element {
    // 渲染所有打开的弹窗
    let modals = MODAL_MANAGER.read().modals.clone();
    rsx! {
        div {
            class: "modal-provider",
            for (id, modal) in modals.iter() {
                if modal.is_open {
                    ModalComponent {
                        modal: modal.clone(),
                        id: id.clone(),
                    }
                }
            }
        }

    }
}
