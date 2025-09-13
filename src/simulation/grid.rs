use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Grid{


    data: HashMap<(i32, i32),Arc<Mutex<Cell>>>,
    size_cell: (i32, i32),
}


struct Cell{
    coordonnee: (i32, i32),
    taille: (i32, i32),
    donne: Vec<Cases>,
}
struct Cases{
    pression : f64,
    mass: f64,
    temperature : f64,
}
impl Grid{
    pub fn where_case_try(&self, case_coordonne:(i32,i32))->Option<(i32,i32)>{
        let coordonne = self.where_case(case_coordonne);
        if self.data.contains_key(&coordonne) {
            Some(coordonne)
        } else {
            None
        }
    }
    #[inline]
    pub fn where_case(&self, case_coordonne:(i32,i32))->(i32,i32){
        (case_coordonne.0/self.size_cell.0,case_coordonne.1/self.size_cell.1)
    }
    pub fn who_cells_by_cases(&self, case_coordonne_left_up:(i32,i32),case_coordonne_right_drown:(i32,i32))->Vec<Arc<Mutex<Cell>>>{
        let mut cells = Vec::new();
        //TODO


    }
}
impl Cell{
    fn get_cases(&self,coordinate: (i32, i32)) -> Option<&Cases>{
        if coordinate.0>= self.taille.0*self.coordonnee.0 &&
            coordinate.0 <self.taille.0*(self.coordonnee.0+1) &&
            coordinate.1 >= self.taille.1*self.coordonnee.1 &&
            coordinate.1 < self.taille.1*(self.coordonnee.1+1){
            Some(&self.donne[(coordinate.0+coordinate.1*self.taille.0 as i32 )as usize])
        }else {
            None
        }

    }
    fn get_cases_mut(&mut self,coordinate: (i32, i32)) -> Option<&mut Cases>{
        if coordinate.0>= self.taille.0*self.coordonnee.0 &&
            coordinate.0 <self.taille.0*(self.coordonnee.0+1) &&
            coordinate.1 >= self.taille.1*self.coordonnee.1 &&
            coordinate.1 < self.taille.1*(self.coordonnee.1+1){
            Some(&mut self.donne[(coordinate.0 + coordinate.1 * self.taille.0 as i32) as usize])
        }else {
            None
        }

    }
}