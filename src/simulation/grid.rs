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