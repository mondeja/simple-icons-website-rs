//! Hex colors sorting

use colorsys::{Hsl, Rgb};
use std::cmp::Ordering;

const GREY_RANGE: u8 = 10;
const BLACK_CUTOFF: u8 = 15;
const WHITE_CUTOFF: u8 = 90;

fn hex_to_tuple(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn hsl_to_upper_hex_string(hsl: &Hsl) -> String {
    Rgb::from(hsl)
        .to_hex_string()
        .trim_start_matches('#')
        .to_uppercase()
}

fn is_gray(hsl: &Hsl) -> bool {
    let l = hsl.lightness() as u8;
    if l <= BLACK_CUTOFF || l >= WHITE_CUTOFF {
        return true;
    }

    let [r, g, b]: [u8; 3] = Rgb::from(hsl).into();

    if g < GREY_RANGE || b < GREY_RANGE {
        return true;
    } else {
        return r <= g + GREY_RANGE
            && b <= g + GREY_RANGE
            && r <= b + GREY_RANGE;
    }
}

pub fn sort_hexes(hexes: &Vec<String>) -> Vec<String> {
    let mut coloreds: Vec<Hsl> = vec![];
    let mut greys: Vec<Hsl> = vec![];

    for hsl in hexes
        .iter()
        .map(|hex| Hsl::from(Rgb::from(hex_to_tuple(hex))))
    {
        if is_gray(&hsl) {
            greys.push(hsl);
        } else {
            coloreds.push(hsl);
        }
    }

    let mut ret: Vec<String> = Vec::with_capacity(hexes.len());
    coloreds.sort_by(|a, b| {
        // sort by hue
        let hue = a.hue() - b.hue();
        if hue != 0 as f64 {
            return if hue > 0 as f64 {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }

        // if hue is the same, sort by saturation
        let sat = a.saturation() - b.saturation();
        return if sat > (0 as f64) {
            Ordering::Greater
        } else if sat < (0 as f64) {
            Ordering::Less
        } else {
            Ordering::Equal
        };
    });
    ret.extend(coloreds.iter().map(|c| hsl_to_upper_hex_string(c)));

    // sort greys by lightness
    greys.sort_by(|a, b| {
        let lightness = a.lightness() - b.lightness();
        return if lightness > (0 as f64) {
            Ordering::Greater
        } else if lightness < (0 as f64) {
            Ordering::Less
        } else {
            Ordering::Equal
        };
    });
    ret.extend(greys.iter().map(|c| hsl_to_upper_hex_string(c)));

    ret
}
