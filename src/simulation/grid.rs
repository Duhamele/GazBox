use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Grid2D{


    pub(crate) data: HashMap<(i32, i32),Arc<Mutex<Cell>>>,
    size_cell: (i32, i32),
}
impl Default for Grid2D {
    fn default() -> Grid2D {
        let mut grid=Grid2D{data: HashMap::new(), size_cell: (16,16)};
        for x in -4..5{
            for y in -4..5{
                grid.data.insert((x,y),Arc::new(Mutex::new(Cell::new(x,y))));
            }
        }


        grid
    }

}


pub struct Cell{
    coordonnee: (i32, i32),
    pub(crate) taille: (i32, i32),
    donne: Vec<Cases>,
}
impl Cell {
    fn new(x:i32, y:i32) -> Cell {
        let mut cell=Cell{coordonnee: (x, y), taille: (16,16), donne: Vec::new()};
        for i in 0..(16*16){
            cell.donne.push(Cases{pression:1.0,mass:1.,temperature:16.})
        }
        cell
    }

}
pub struct Cases{
    pub(crate) pression : f64,
    pub(crate) mass: f64,
    pub(crate) temperature : f64,
}
impl Grid2D{
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
        let (x_start,y_start)=self.where_case(case_coordonne_left_up);
        let (x_end,y_end)=self.where_case(case_coordonne_right_drown);
        for x in x_start..=x_end{
            for y in y_start..=y_end{
                match self.data.get(&(x,y)) {
                    Some(cell) => {
                        cells.push(Arc::clone(cell));
                    }
                    None => {}
                }
            }
        }
        cells
    }
}
impl Cell{
    pub(crate) fn get_cases(&self, coordinate: (i32, i32)) -> Option<&Cases>{
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