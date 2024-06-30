use std::cell::RefCell;
use std::rc::Rc;

use eframe::egui::{CentralPanel, Color32, Context, Frame, RichText, Ui};

use crate::cheese::gui::colors::MENU_SHADOW_COLOR;

pub(super) enum MenuEntry {
    Execute {
        name: String,
        function: fn() -> anyhow::Result<()>,
    },
    State {
        name: String,
        state: Rc<RefCell<bool>>,
    },
    Redirect {
        name: String,
        to_menu_id: u32,
    },
}

impl MenuEntry {
    pub fn name(&self) -> &str {
        match self {
            MenuEntry::Execute { name, .. } => name.as_str(),
            MenuEntry::State { name, .. } => name.as_str(),
            MenuEntry::Redirect { name, .. } => name.as_str(),
        }
    }

    pub fn draw_inner(&self, ui: &mut Ui, is_selected: bool) {
        let mut text = RichText::new(self.name()).size(17.);
        if is_selected {
            text = text
                .background_color(MENU_SHADOW_COLOR)
                .color(Color32::WHITE)
                .italics();
        }
        match self {
            MenuEntry::Execute { name, .. } | MenuEntry::Redirect { name, .. } => {
                ui.label(text);
            }
            MenuEntry::State { name, state } => {
                ui.checkbox(&mut state.borrow_mut(), text);
            }
        }
    }

    // returns to which menu to redirect to, if any
    pub fn draw(&self, ui: &mut Ui, is_selected: bool) {
        if is_selected {
            let available_width = ui.available_width();
            ui.allocate_ui([available_width, 0.0].into(), |ui| {
                Frame::none()
                    .fill(MENU_SHADOW_COLOR)
                    .show(ui, |ui| self.draw_inner(ui, is_selected))
            });
        } else {
            self.draw_inner(ui, is_selected)
        }
    }
}

pub(super) struct MenuDefinition {
    pub(crate) name: String,
    pub(crate) entries: Vec<MenuEntry>,
    pub(crate) selected: usize,
}

impl MenuDefinition {
    const BACK_TEXT: &'static str = "<< Back";

    pub fn new(name: &str) -> Self {
        MenuDefinition {
            name: name.to_string(),
            entries: vec![MenuEntry::Redirect {
                name: Self::BACK_TEXT.to_string(),
                to_menu_id: 0,
            }],
            selected: 0,
        }
    }

    pub fn add_entry(&mut self, entry: MenuEntry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&self, name: &str) -> Option<&MenuEntry> {
        self.entries.iter().find(|&entry| entry.name() == name)
    }

    pub fn replace_back_link(&mut self, new_to_menu_id: u32) {
        if let Some(MenuEntry::Redirect { name, to_menu_id }) = self.entries.get_mut(0) {
            if name != Self::BACK_TEXT {
                return;
            }
            *to_menu_id = new_to_menu_id;
        }
    }

    pub fn draw(&self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for (i, entry) in self.entries.iter().enumerate() {
                    entry.draw(ui, i == self.selected);
                }
            });
        });
    }
}
