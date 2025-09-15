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