use eframe::egui::*;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_END};
use crate::cheese::classes::ped::CPed;

#[derive(Default)]
struct TheCheese {
    window_hidden: bool,
    just_pressed_end: bool,
    seatbelt: bool,
}

impl TheCheese {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(Color32::from_rgb(80, 20, 255));
        cc.egui_ctx.set_visuals(visuals);

        let font_data = include_bytes!("../../../assets/kimberley bl.otf");
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "cheese_font".to_owned(),
            FontData::from_static(font_data),
        );

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

        Self {
            window_hidden: false,
            ..Default::default()
        }
    }

    unsafe fn handle_cheats(&self) -> anyhow::Result<()> {
        self.seatbelt()
    }

    unsafe fn seatbelt(&self) -> anyhow::Result<()> {
        if let Some(local_player) = CPed::local_player() {
            local_player.set_seatbelt(self.seatbelt)?;
        }
        Ok(())
    }
}

impl eframe::App for TheCheese {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
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

        unsafe {
            self.handle_cheats().unwrap();
        }

        ctx.request_repaint();

        if self.window_hidden {
            return;
        }

        egui_extras::install_image_loaders(ctx);
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading(RichText::new(format!("CH33S3 V{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"))).size(30.).heading());
            });
        });

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Made by imkx"));
            });
        });

        TopBottomPanel::bottom("keybind_footer").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Press END to toggle menu"));
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            fn add_cheat_row(ui: &mut Ui, name: &str, enabled: &mut bool) {
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(true, |ui| {
                        ui.checkbox(enabled, name);
                    })
                });
            }
            add_cheat_row(ui, "Seatbelt", &mut self.seatbelt);
        });
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}

pub unsafe fn run_graphics() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            //.with_mouse_passthrough(true)
            .with_always_on_top()
            .with_inner_size([300.0, 600.0])
            .with_position([20.0, 20.0])
            .with_titlebar_shown(false)
            .with_titlebar_buttons_shown(false)
            .with_resizable(false)
            .with_decorations(false)
            .with_transparent(true)
            .with_visible(true),
        ..Default::default()
    };
    eframe::run_native(
        "The Cheese",
        options,
        Box::new(|cc| Box::new(TheCheese::new(cc))),
    )
        .expect("OUCH");
}
