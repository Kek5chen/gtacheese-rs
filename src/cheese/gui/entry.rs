use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Once;
use std::time::Duration;

use eframe::egui::*;
use eframe::CreationContext;
use windows::Win32::Foundation::{BOOL, COLORREF, HWND, LPARAM, MAX_PATH, TRUE};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_END};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, GetWindowThreadProcessId, SetLayeredWindowAttributes, LWA_ALPHA,
};

use crate::cheese::classes::ped::CPed;
use crate::cheese::classes::wanted::CWanted;
use crate::cheese::gui::colors::MENU_PRIMARY_COLOR;
use crate::cheese::gui::main_elements::{draw_credits_footer, draw_header, draw_keybinds_footer};
use crate::cheese::gui::menu_definition::MenuDefinition;
use crate::cheese::gui::menus::MAIN_MENU_ID;

#[derive(Default)]
pub(super) struct TheCheese {
    pub(crate) window_hidden: bool,
    pub(crate) just_pressed_end: bool,
    pub(crate) seatbelt: Rc<RefCell<bool>>,
    pub(crate) godmode: Rc<RefCell<bool>>,
    pub(crate) never_wanted: Rc<RefCell<bool>>,
    pub(crate) current_menu_id: u32,
    pub(crate) menu_definitions: HashMap<u32, MenuDefinition>,
}

impl TheCheese {
    fn new(cc: &CreationContext<'_>) -> Self {
        Self::setup_visuals(cc);
        Self::setup_font(cc);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut me = Self {
            window_hidden: false,
            current_menu_id: MAIN_MENU_ID,
            ..Default::default()
        };

        me.setup_menus();
        me
    }

    fn setup_visuals(cc: &CreationContext) {
        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(MENU_PRIMARY_COLOR);
        cc.egui_ctx.set_visuals(visuals);
    }

    fn setup_font(cc: &CreationContext) {
        let font_data = include_bytes!("../../../assets/kimberley bl.otf");
        let mut fonts = FontDefinitions::default();

        fonts
            .font_data
            .insert("cheese_font".to_owned(), FontData::from_static(font_data));

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "cheese_font".to_string());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("cheese_font".to_string());

        cc.egui_ctx.set_fonts(fonts);
    }

    unsafe fn handle_state(&self) -> anyhow::Result<()> {
        let _ = self.seatbelt();
        let _ = self.godmode();
        let _ = self.never_wanted();

        Ok(())
    }

    unsafe fn seatbelt(&self) -> anyhow::Result<()> {
        if let Some(local_player) = CPed::local_player() {
            local_player.set_seatbelt(*self.seatbelt.borrow())?;
        }
        Ok(())
    }

    unsafe fn godmode(&self) -> anyhow::Result<()> {
        if *self.godmode.borrow() {
            if let Some(local_player) = CPed::local_player() {
                local_player.set_health(100000.)?;
            }
        } else if let Some(local_player) = CPed::local_player() {
            if let Some(max_health) = local_player.get_max_health() {
                if let Some(health) = local_player.get_health() {
                    if health > max_health {
                        local_player.set_health(max_health)?;
                    }
                }
            }
        }
        Ok(())
    }

    unsafe fn never_wanted(&self) -> anyhow::Result<()> {
        if *self.never_wanted.borrow() {
            if let Some(wanted) = CWanted::get_local_wanted() {
                wanted.set_wanted_level(0)?;
            }
        }
        Ok(())
    }

    fn handle_window_hiding(&mut self) {
        unsafe {
            if GetAsyncKeyState(VK_END.0 as i32) != 0 {
                if !self.just_pressed_end {
                    self.just_pressed_end = true;
                    self.window_hidden = !self.window_hidden;
                }
            } else {
                self.just_pressed_end = false;
            }
        }
    }
}

impl eframe::App for TheCheese {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        init_window_visibility_fix();

        ctx.request_repaint_after(Duration::from_millis(1000 / 60));

        self.handle_window_hiding();

        unsafe {
            self.handle_state().unwrap();
        }

        if self.window_hidden {
            return;
        }

        draw_header(ctx);
        draw_credits_footer(ctx);
        draw_keybinds_footer(ctx);
        self.update_cheat_menu();
        self.draw_cheat_menu(ctx);
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}

pub unsafe fn run_graphics() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_visible(true)
            .with_mouse_passthrough(true)
            .with_always_on_top()
            .with_inner_size([300.0, 600.0])
            .with_position([20.0, 20.0])
            .with_titlebar_shown(false)
            .with_titlebar_buttons_shown(false)
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "The Cheese",
        options,
        Box::new(|cc| Box::new(TheCheese::new(cc))),
    )
    .expect("OUCH");
}

static INIT_WINDOW_VISIBILITY_FIX: Once = Once::new();

pub fn init_window_visibility_fix() {
    INIT_WINDOW_VISIBILITY_FIX.call_once(|| {
        if let Err(e) = fix_window_visibility() {
            log::error!("Failed to fix window visibility: {:?}", e);
        }
    });
}

fn fix_window_visibility() -> windows::core::Result<()> {
    unsafe { EnumWindows(Some(enum_window_proc), LPARAM(0)) }
}

extern "system" fn enum_window_proc(hwnd: HWND, _: LPARAM) -> BOOL {
    let mut class_name_buffer = [0u16; MAX_PATH as usize];

    unsafe {
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        if process_id == GetCurrentProcessId() {
            if GetWindowTextW(hwnd, &mut class_name_buffer) == 0 {
                return TRUE;
            }

            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        }
    }
    TRUE
}
