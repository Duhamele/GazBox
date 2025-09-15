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
use eframe::emath::Vec2;
use egui::{Context, Painter, Pos2, Ui};
use crate::graphique::gaz::Graphique;

pub struct Tooltip {
    tooltip: String,
}

impl Tooltip {
    pub fn new(tooltip: String) -> Self {
        Self { tooltip }
    }
    pub fn show(& self, ui: &mut Ui, mut pos:Pos2) ->Pos2 {
        let galley = ui.painter().layout_no_wrap(
            self.tooltip.to_owned(),
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );

        let rect = egui::Rect::from_min_size(
            pos,
            galley.size() + egui::vec2(8.0, 8.0),
        );
        pos+=egui::vec2(0.,galley.size().y).into();
        ui.painter().rect_filled(rect, 6.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 200));
        ui.painter().galley(rect.min + egui::vec2(4.0, 4.0), galley);
        pos
    }

}
#[derive(Default)]
pub struct ManagerTooltip {
    tooltip: Vec<Tooltip>,
}
impl ManagerTooltip {
    pub fn add(&mut self, tooltip: Tooltip) {
        self.tooltip.push(tooltip);
    }
    pub fn show(&mut self,ui:&mut Ui ) {
        if let Some(mouse_pos) = ui.ctx().pointer_hover_pos(){
            let mut pos = mouse_pos+Vec2::new(4.0,4.0);
            for tooltip in &self.tooltip{
                pos=tooltip.show(ui,pos);
            }

        }
    }
    pub fn clear(&mut self) {
        self.tooltip.clear();
    }
}


