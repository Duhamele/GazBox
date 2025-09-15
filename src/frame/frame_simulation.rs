use egui::{Context, };
use super::super::simulation::grid::Grid2D;

pub struct FrameSimulation {
    offset: egui::Vec2,
    zoom:f32,
    grid: Grid2D,
}
impl Default for FrameSimulation {
    fn default() -> Self {
        FrameSimulation{
            offset: egui::Vec2::new(0.,0.),
            zoom: 10.0,
            grid: Grid2D::default(),
        }
    }
}
impl FrameSimulation {
    pub fn show(&mut self,ctx:&Context){
        egui::CentralPanel::default().show(ctx, |ui|{
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::drag());

            // Gérer le drag (déplacement de la map)
            if response.dragged() {
                self.offset += response.drag_delta();
                ctx.request_repaint();
            }
            if response.hovered() {
                if ctx.input(|i| i.scroll_delta.y != 0.0) {
                    let scroll = ctx.input(|i| i.scroll_delta.y);
                    self.zoom *= (1.0 + scroll * 0.01).clamp(0.5, 5.0);
                    ctx.request_repaint();
                }
            }


            let cell_size = self.zoom;
            for ((x, y), cell) in &self.grid.data {
                let cell = cell.lock().unwrap();

                for iy in 0..cell.taille.1 {
                    for ix in 0..cell.taille.0 {
                        if let Some(c) = cell.get_cases((ix as i32, iy as i32)) {

                            let color = egui::Color32::from_rgb(0,120,32);

                            // Position de la case dans l’espace
                            let pos = egui::pos2(
                                (*x as f32 * cell.taille.0 as f32 + ix as f32) * cell_size
                                    + self.offset.x,
                                (*y as f32 * cell.taille.1 as f32 + iy as f32) * cell_size
                                    + self.offset.y,
                            );

                            let rect = egui::Rect::from_min_size(
                                pos,
                                egui::vec2(cell_size, cell_size),
                            );

                            painter.rect_filled(rect, 0.0, color);
                            let id = ui.id().with((*x, *y, ix, iy));
                            let response = ui.interact(rect, id, egui::Sense::hover());

                            if response.hovered() {
                                response.on_hover_ui(|ui| {
                                    ui.label(format!("Température: {:.2}", c.temperature));
                                    ui.label(format!("Pression: {:.2}", c.pression));
                                    ui.label(format!("Masse: {:.2}", c.mass));
                                });
                            }
                        }
                    }
                }
            }

        });
    }
}