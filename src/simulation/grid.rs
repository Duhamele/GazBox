use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Grid{


    data: HashMap<(i32, i32),Arc<Cell>>,
    size_cell: (usize, usize),
}


struct Cell{
    coordonnee: (i32, i32),
    taille: (usize, usize),
    donne: [Cases],
}
struct Cases{
    pression : f64,
    mass: f64,
    temperature : f64,
}
impl Grid{
    fn get_cases(&self,coordinate: (i32, i32)) -> Option<&Cases>{
        let cood_cell=(coordinate.0 / self.size_cell.0 as i32 , coordinate.1 / self.size_cell.1 as i32 );
        match self.data.get(&cood_cell){
            Some(cell)=>{
                let cood_cases=(coordinate.0 % self.size_cell.0 as i32 , coordinate.1 % self.size_cell.1 as i32 );
                cell.get_cases(cood_cases)
            }
            None=>{
                return None;
            }
        }

    }
}
impl Cell{
    fn get_cases(&self,coordinate: (i32, i32)) -> Option<&Cases>{
        if coordinate.0>= 0 && coordinate.0 <self.taille.0 as i32 && coordinate.1 >= 0 && coordinate.1 < self.taille.1 as i32{
            Some(&self.donne[(coordinate.0+coordinate.1*self.taille.0 as i32 )as usize])
        }else {
            None
        }

    }
}