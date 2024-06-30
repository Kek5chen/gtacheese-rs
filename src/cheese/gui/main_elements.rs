use eframe::egui::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::cheese::gui::menu_definition::MenuEntry;
use crate::cheese::gui::menus::MAIN_MENU_ID;

use super::entry::TheCheese;

pub(super) fn draw_header(ctx: &Context) {
    TopBottomPanel::top("header").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading(
                RichText::new(format!(
                    "CH33S3 V{}.{}",
                    env!("CARGO_PKG_VERSION_MAJOR"),
                    env!("CARGO_PKG_VERSION_MINOR")
                ))
                .size(30.)
                .heading(),
            );
        });
    });
}

pub(super) fn draw_credits_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("Made by imkx"));
        });
    });
}

pub(super) fn draw_keybinds_footer(ctx: &Context) {
    TopBottomPanel::bottom("keybind_footer").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("Press END to toggle menu"));
        });
    });
}

impl TheCheese {
    pub(super) fn update_cheat_menu(&mut self) {
        // TODO: Make this multi threaded so it isn't so slow and more responsive
        unsafe {
            if (GetAsyncKeyState(VK_NUMPAD2.0 as i32) & 1) == 1
                || (GetAsyncKeyState(VK_J.0 as i32) & 1) == 1
            {
                let cur_sel = self.get_current_selection();
                self.set_current_selected_entry(cur_sel + 1);
            }
            if (GetAsyncKeyState(VK_NUMPAD8.0 as i32) & 1) == 1
                || (GetAsyncKeyState(VK_K.0 as i32) & 1) == 1
            {
                let cur_sel = self.get_current_selection();
                if cur_sel != 0 {
                    self.set_current_selected_entry(cur_sel - 1);
                }
            }

            if (GetAsyncKeyState(VK_NUMPAD5.0 as i32) & 1) == 1
                || (GetAsyncKeyState(VK_L.0 as i32) & 1) == 1
            {
                self.press_button();
            }
        }
    }
    pub(super) fn draw_cheat_menu(&mut self, ctx: &Context) {
        if let Some(menu) = self.menu_definitions.get(&self.current_menu_id) {
            menu.draw(ctx);
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading(
                        RichText::new("Error... No menus exist")
                            .heading()
                            .color(Color32::RED),
                    );
                });
            });

            if self.current_menu_id != MAIN_MENU_ID {
                self.current_menu_id = MAIN_MENU_ID;
            }
        }
    }

    fn get_current_selection(&self) -> usize {
        self.menu_definitions
            .get(&self.current_menu_id)
            .map(|menu| menu.selected)
            .unwrap_or_default()
    }

    fn get_current_max_entry(&self) -> usize {
        self.menu_definitions
            .get(&self.current_menu_id)
            .map(|menu| menu.entries.len())
            .unwrap_or_default()
    }

    fn set_current_selected_entry(&mut self, entry: usize) {
        if let Some(menu) = self.menu_definitions.get_mut(&self.current_menu_id) {
            if entry >= menu.entries.len() {
                return;
            }

            menu.selected = entry;
        }
    }

    fn press_button(&mut self) {
        if let Some(menu) = self.menu_definitions.get_mut(&self.current_menu_id) {
            if let Some(entry) = menu.entries.get(menu.selected) {
                match entry {
                    MenuEntry::Execute { function, name } => match function() {
                        Ok(()) => {}
                        Err(e) => log::warn!("Function {} exited with error: {}", name, e),
                    },
                    MenuEntry::State { state, .. } => {
                        let mut state = state.borrow_mut();
                        *state = !*state;
                    }
                    MenuEntry::Redirect { to_menu_id, .. } => {
                        self.current_menu_id = *to_menu_id;
                    }
                }
            }
        }
    }
}
