use easter::{gregorian_easter, julian_easter};

fn main() {
    let year = 2019;
    let (month, day) = gregorian_easter(year);
    let (jmonth, jday) = julian_easter(year);
    println!("Gregorian Easter in {}: {} {}", year, day, month);
    println!("Julian Easter in {}: {} {}", year, jday, jmonth);
}
