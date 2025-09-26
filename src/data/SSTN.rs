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
use std::fmt::{write, Display, Formatter};
use crate::data::time::{DurationBasic, TimeBasic};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct SSTN_Time{
    nano_sec_ap:i64
}
impl Display for SSTN_Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut time=self.nano_sec_ap;
        let nano=time%1000;
        time/=1000;
        let micros=time%1000;
        time/=1000;
        let millis=time%1000;
        time/=1000;
        let seconds=time%60;
        time/=60;
        let minutes=time%60;
        time/=60;
        let hours=time%24;
        time/=24;
        let days=time;
        write!(f, "{}:{:02}:{:02}:{:02}:{:03}:{:03}:{:03}", days, hours, minutes, seconds, millis, micros, nano)
    }
}
impl TimeBasic for SSTN_Time{
    type ErrorTime = ();

    fn now_zero() -> Self {
        SSTN_Time{nano_sec_ap: 0}
    }

    fn try_str(string: String) -> Result<Self, Self::ErrorTime> {
        panic!("not yet implemented")

    }


}
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct SSTN_Duration {
    nano_sec:i64
}
impl Display for SSTN_Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut time=self.nano_sec;
        let nano=time%1000;
        time/=1000;
        let micros=time%1000;
        time/=1000;
        let millis=time%1000;
        time/=1000;
        let seconds=time%60;
        time/=60;
        let minutes=time%60;
        time/=60;
        let hours=time%24;
        time/=24;
        let days=time;
        write!(f, "{}:{:02}:{:02}:{:02}:{:03}:{:03}:{:03}", days, hours, minutes, seconds, millis, micros, nano)
    }
}
impl DurationBasic for SSTN_Duration {
    type Time = SSTN_Time;

    fn is_null(&self) -> bool {
        self.nano_sec == 0
    }

    fn is_positive(&self) -> bool {
        self.nano_sec >= 0
    }

    fn is_negative(&self) -> bool {
        self.nano_sec <= 0
    }

}
impl std::ops::Add for SSTN_Duration {
    type Output = SSTN_Duration;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            nano_sec: self.nano_sec + rhs.nano_sec
        }
    }
}
impl std::ops::Sub for SSTN_Duration {
    type Output = SSTN_Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            nano_sec: self.nano_sec - rhs.nano_sec
        }
    }
}
impl<Rhs> std::ops::Div<Rhs> for SSTN_Duration
    where Rhs: Into<i64> {
    type Output = SSTN_Duration;

    fn div(self, rhs: Rhs) -> Self::Output {
        Self{
            nano_sec: self.nano_sec / rhs.into()
        }
    }
}
impl<Rhs> std::ops::Mul<Rhs> for SSTN_Duration
where Rhs: Into<i64> {
    type Output = SSTN_Duration;
    fn mul(self, rhs: Rhs) -> Self::Output {
        Self{
            nano_sec: self.nano_sec * rhs.into()
        }
    }
}
struct TypeName<T>(T);
impl<Lhs> std::ops::Mul<SSTN_Duration> for TypeName<Lhs>
    where Lhs: Into<i64>+Copy {
    type Output = SSTN_Duration;

    fn mul(self, rhs: SSTN_Duration) -> Self::Output {
        SSTN_Duration{
            nano_sec:self.0.into() * rhs.nano_sec
        }
    }
}