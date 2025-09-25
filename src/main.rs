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

mod frame;
mod simulation;
mod graphique;
mod data;

use eframe::egui;
use std::f32::consts::PI;

struct Marker {
    x: f32,
    y: f32,
    angle: f32,
}

struct MyApp {
    t: f32,
    markers: Vec<Marker>,
}

// On implémente maintenant directement `eframe::App`
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.t += 0.05;

        // Déplacer les marqueurs en cercle
        for m in &mut self.markers {
            m.angle += 0.02;
            m.x = 200.0 + (m.angle).sin() * 100.0;
            m.y = 200.0 + (m.angle).cos() * 100.0;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            // Fond "map"
            painter.rect_filled(response.rect, 0.0, egui::Color32::LIGHT_GRAY);

            // Dessiner chaque marqueur
            for m in &self.markers {
                let pos = egui::pos2(m.x, m.y);
                painter.circle_filled(pos, 10.0, egui::Color32::RED);

                // Menu contextuel
                if response.hover_pos().map_or(false, |mouse| mouse.distance(pos) < 15.0) {
                    egui::popup::show_tooltip(ctx, response.id, |ui| {
                        if ui.button("Option 1").clicked() {
                            println!("Option 1 pour marqueur");
                        }
                        if ui.button("Option 2").clicked() {
                            println!("Option 2 pour marqueur");
                        }
                    });
                }
            }

        });
        
        ctx.request_repaint(); // redessiner en continu
    }
}
use frame::frame_main;
fn main() {
    env_logger::init();

    let app = frame_main::BoxGaz::default();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Map Alpha egui", native_options, Box::new(|_cc| Box::new(app)));
}
