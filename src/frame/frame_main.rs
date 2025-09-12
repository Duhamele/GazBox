use eframe::Frame;
use egui::{Context, ViewportCommand};
use egui;

pub struct BoxGaz{
    popup_quit: bool,
}
impl Default for BoxGaz {
    fn default() -> Self {
        BoxGaz{
            popup_quit: false,
        }
    }
}
impl eframe::App for BoxGaz {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        
        egui::Window::new("Menu").show(ctx, |ui| {

            if ui.button("Quit").clicked() {
                println!("Quit!");
                self.popup_quit = true;
            }
        });
        if self.popup_quit {
            egui::Window::new("Confirmer la fermeture")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // centré
                .show(ctx, |ui| {
                    ui.label("Voulez-vous vraiment quitter l'application ?");
                    ui.horizontal(|ui| {
                        if ui.button("Oui").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                        if ui.button("Non").clicked() {
                            self.popup_quit = false; // annule
                        }
                    });
                });
        }

        ctx.request_repaint();
    }
}