/*
Copyright  Elie Duhamel
15 septembre 2025

elie.duhamel@kivas.fr

Ce logiciel est un programme informatique servant à simuler des gaz.

Ce logiciel est régi par la licence CeCILL soumise au droit français et
respectant les principes de diffusion des logiciels libres. Vous pouvez
utiliser, modifier et/ou redistribuer ce programme sous les conditions
de la licence CeCILL telle que diffusée par le CEA, le CNRS et l'INRIA
sur le site "http://www.cecill.info".

En contrepartie de l'accessibilité au code source et des droits de copie,
de modification et de redistribution accordés par cette licence, il n'est
offert aux utilisateurs qu'une garantie limitée.  Pour les mêmes raisons,
seule une responsabilité restreinte pèse sur l'auteur du programme,  le
titulaire des droits patrimoniaux et les concédants successifs.

A cet égard  l'attention de l'utilisateur est attirée sur les risques
associés au chargement,  à l'utilisation,  à la modification et/ou au
développement et à la reproduction du logiciel par l'utilisateur étant
donné sa spécificité de logiciel libre, qui peut le rendre complexe à
manipuler et qui le réserve donc à des développeurs et des professionnels
avertis possédant  des  connaissances  informatiques approfondies.  Les
utilisateurs sont donc invités à charger  et  tester  l'adéquation  du
logiciel à leurs besoins dans des conditions permettant d'assurer la
sécurité de leurs systèmes et ou de leurs données et, plus généralement,
à l'utiliser et l'exploiter dans les mêmes conditions de sécurité.

Le fait que vous puissiez accéder à cet en-tête signifie que vous avez
pris connaissance de la licence CeCILL, et que vous en avez accepté les
termes.
 */


use egui::{Context, };
use crate::graphique::tooltip::{ManagerTooltip, Tooltip};
use crate::simulation::data::ParamtreBoxGaz;
use super::super::simulation::grid::Grid2D;

pub struct FrameSimulation {
    offset: egui::Vec2,
    zoom:f32,
    grid: Grid2D,
    manager_tooltip:ManagerTooltip,
}
impl Default for FrameSimulation {
    fn default() -> Self {
        FrameSimulation{
            offset: egui::Vec2::new(0.,0.),
            zoom: 10.0,
            grid: Grid2D::default(),
            manager_tooltip:ManagerTooltip::default(),
        }
    }
}
impl FrameSimulation {
    pub fn show(&mut self,ctx:&Context,param:&ParamtreBoxGaz){
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
                self.manager_tooltip.clear();

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
                                self.manager_tooltip.add(Tooltip::new(format!("Température: {:.2}", c.temperature)))


                            }
                        }
                    }
                }
            }
            self.manager_tooltip.show(ui);


        });
    }
}