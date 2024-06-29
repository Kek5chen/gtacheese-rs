use crate::cheese::gui::colors::{MENU_PRIMARY_COLOR, MENU_SHADOW_COLOR};
use eframe::egui::{CentralPanel, Context, CursorIcon, Frame, Ui};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) enum MenuEntry {
    Execute {
        name: String,
        function: fn(),
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

    pub fn draw_inner(&self, ui: &mut Ui) {
        match self {
            MenuEntry::Execute { name, function } => {
                ui.label(name);
            }
            MenuEntry::State { name, state } => {
                ui.checkbox(&mut state.borrow_mut(), name);
            }
            MenuEntry::Redirect { name, to_menu_id } => {
                ui.label(name).on_hover_cursor(CursorIcon::PointingHand);
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
                    .show(ui, |ui| self.draw_inner(ui))
            });
        } else {
            self.draw_inner(ui)
        }
    }
}

pub(super) struct MenuDefinition {
    pub(crate) name: String,
    pub(crate) entries: Vec<MenuEntry>,
    pub(crate) selected: usize,
}

impl MenuDefinition {
    pub fn new(name: &str) -> Self {
        MenuDefinition {
            name: name.to_string(),
            entries: vec![MenuEntry::Redirect {
                name: "<< Back".to_string(),
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

    pub fn draw(&self, ctx: &Context) {
        CentralPanel::default()
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    for (i, entry) in self.entries.iter().enumerate() {
                        entry.draw(ui, i == self.selected);
                    }
                });
            });
    }
}
