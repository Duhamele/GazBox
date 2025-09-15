use eframe::Frame;
use egui::{Context, ViewportCommand};
use egui;
use crate::frame::frame_simulation::FrameSimulation;

pub struct BoxGaz{
    popup_quit: bool,



    //finalion
    simulation:FrameSimulation,
}
impl Default for BoxGaz {
    fn default() -> Self {
        BoxGaz{
            popup_quit: false,
            simulation:FrameSimulation::default(),
        }
    }
}
impl BoxGaz{
    pub fn popup_quit(& mut self,ctx: &Context) {
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
}
impl eframe::App for BoxGaz {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::SidePanel::left("panneau_gauche").show(ctx, |ui| {
            ui.heading("Menu");
            if ui.button("Quitte").clicked() {
                println!("Option 2 cliquée");
                self.popup_quit = true;
            }
        });
        self.simulation.show(ctx);
        



        if self.popup_quit {
            self.popup_quit(ctx);
        }

        ctx.request_repaint();
    }
}