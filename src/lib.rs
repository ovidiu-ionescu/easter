//! # Easter
//! `easter` contains functions for calculating the Easter dates for both
//! Gregorian and Julian calendars.

/// Calculates the date of Easter for a given year
///
/// Algorithm from Knuth TAOCP vol 1, 1.3.2, example 14
///
/// # Examples
/// 
/// Easter in 2019 is on the 21<sup>st</sup> of April
/// ```
/// assert_eq!((4, 21), easter::gregorian_easter(2019));
/// ```
///
pub fn gregorian_easter(year: i32) -> (i32, i32) {
    let golden_number = (year % 19) + 1;
    let century = (year / 100) + 1;
    // Correction for leap years dropped
    let correction_x = (3 * century) / 4 - 12;
    // Correction for moon orbit
    let correction_z = (8 * century +5) / 25 - 5;
    let sunday = 5 * year / 4 - correction_x - 10;
    // epact is for getting full moon
    let mut epact = (11 * golden_number + 20 + correction_z - correction_x) % 30;
    epact = if epact < 0 { epact + 30 }  else { epact };
    epact = if golden_number > 11 || epact == 24 { epact + 1} else { epact };
    let mut full_moon = 44 - epact;
    // it must be after March 21, Spring equinox
    if full_moon < 21 {
        full_moon = full_moon + 30;
    }
    let easter_sunday = full_moon + 7 - (sunday + full_moon) % 7;
    if full_moon > 31 { 
        (4, easter_sunday - 31) 
    } else {
        (3, easter_sunday)
    }
}

/// Calculates the date of Orthodox Easter for a given year
/// 
/// # Examples
/// 
/// Orthodox Easter in 2019 is on the 28<sup>th</sup> of April
/// ```
/// assert_eq!((4, 28), easter::julian_easter(2019));
/// ```
pub fn julian_easter(year: i32) -> (i32, i32) {
    let a = year % 4;
    let b = year % 7;
    let c = year % 19;
    let d = (19 * c + 15) % 30;
    let e = (2 * a + 4 * b - d + 34) % 7;
    let month = (d + e + 114) / 31;
    let day = ((d + e + 114) % 31) + 1;
    // add 13 day to translate to Gregorian
    match month {
        3 => if day + 13 > 31 { (4, (day + 13) % 31) } else { (month, day + 13) },
        4 => if day +  13 > 30 { (5, (day + 13) % 30) } else { (month, day + 13) },
        _ => (month, day + 13)
    }
}