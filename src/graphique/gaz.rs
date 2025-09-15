use egui::{Context, Painter};
///
/// 
///
///
///
pub trait Graphique {
    ///
    ///
    ///
    ///
    fn show(&self,ctx:&Context,paintre:&Painter,zoom:f32,offset:egui::Vec2,);


}