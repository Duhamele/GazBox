/*
Copyright  Elie Duhamel
25 septembre 2025

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
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait TimeBasic:Sized+Debug+Display+ToString{
    type ErrorTime;
    fn now_zero()->Self;
    fn try_str(string: String)->Result<Self,Self::ErrorTime>;
}
pub trait TimeOperateur<D>:TimeBasic
+ AddAssign<D>
+ SubAssign<D>
+ Add<D, Output = Self>
+Sub<D, Output = Self>
where
    D: DurationBasic<Time=Self>{}

pub trait TimeCmp:TimeBasic
+ PartialEq
+ PartialOrd
+Eq
+Ord{}
pub trait Time<D>:TimeCmp+TimeOperateur<D>
where D:DurationBasic<Time=Self>{}
pub trait DurationBasic:Debug+Display+ToString+ Sized{
    type Time: TimeBasic;
    fn is_null(&self)->bool;
    fn is_positive(&self)->bool;
    fn is_negative(&self)->bool;
}
pub trait DurationOperateur<Number>:DurationBasic
+Add
+AddAssign
+Sub
+SubAssign
+Mul<Number>
+MulAssign<Number>
+Div<Number>
+DivAssign<Number>
where Number:Mul<Self,Output=Self>
{}

pub trait DurationCmp:DurationBasic
+Ord
+PartialOrd
+Eq
+PartialEq{}
pub trait Duration<Number>:DurationOperateur<Number>
+DurationCmp
where Number:Mul<Self,Output=Self>{}