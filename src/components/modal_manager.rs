use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use std::collections::HashMap;
use tracing_subscriber::field::debug;
use uuid::Uuid;

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
    position: Option<(f64, f64)>, // 仅Popover需要
}

pub struct ModalManager {
    modals: HashMap<String, Modal>,
    levels: HashMap<u8, Vec<String>>, // 层级管理
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
    pub fn add_modal(&mut self, content: Element) -> String {
        self.add_generic_modal(ModalType::Modal, content, 1, None)
    }

    // 添加Dialog
    pub fn add_dialog(&mut self, content: Element) -> String {
        self.add_generic_modal(ModalType::Dialog, content, 2, None)
    }

    // 添加Message
    pub fn add_message(&mut self, content: Element) -> String {
        self.add_generic_modal(ModalType::Message, content, 3, None)
    }

    // 添加Popover
    pub fn add_popover(&mut self, content: Element, position: (f64, f64)) -> String {
        self.add_generic_modal(ModalType::Popover, content, 4, Some(position))
    }
    // 通用的添加弹窗方法
    pub fn add_generic_modal(
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

        // 管理层级
        self.levels
            .entry(level)
            .or_insert(Vec::new())
            .push(id.clone());

        id
    }

    // 打开弹窗
    pub fn open_modal(&mut self, id: &str) {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.is_open = true;
        }
    }

    // 关闭弹窗
    pub fn close_modal(&mut self, id: &str) {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.is_open = false;
        }
    }

    // 销毁弹窗
    pub fn destroy_modal(&mut self, id: &str) {
        if let Some(modal) = self.modals.remove(id) {
            if let Some(level_modals) = self.levels.get_mut(&modal.level) {
                level_modals.retain(|modal_id| modal_id != id);
            }
            // 更新DOM树中的位置
            println!("Destroying modal with id: {}", id);
            // ...
        }
    }

    // 根据层级关闭所有弹窗
    pub fn destory_all_modals_by_level(&mut self, level: u8) {
        if let Some(level_modals) = self.levels.get(&level) {
            let modals_to_close = level_modals.clone();
            for id in modals_to_close {
                self.close_modal(&id);
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
fn ModalComponent(modal: Modal) -> Element {
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
            style: "{style}",
            div { class: "modal-content", {modal.content.clone()} }
        }
    }
}

#[component]
pub fn ModalManagerProvider() -> Element {
    let modal_manager = use_context::<Signal<ModalManager>>();
    // let mut modal_manager = modal_manager.clone();

    use_effect({
        move || {
            let window = window().expect("no global `window` exists");
            let closure = Closure::wrap(Box::new({
                move || {
                    let mut modal_manager_write = modal_manager.clone();
                    modal_manager_write.write().destory_all_modals_by_level(4);
                }
            }) as Box<dyn FnMut()>);
            window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }
    });

    // 渲染所有打开的弹窗
    let modals = modal_manager.read().modals.clone();
    rsx! {
        div {
            class: "modal-provider",
            for (id, modal) in modals.iter() {
                if modal.is_open {
                    ModalComponent {
                        modal: modal.clone(),
                    }
                }
            }
        }

    }
}
