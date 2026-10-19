//! Zoom: „Optimale Größe“ (ins Fenster einpassen, nie vergrößern) oder ein
//! fester Faktor. Stufen wie Eye of GNOME: 1/4 … 1 … 20, Schritt ×1,25.

use crate::i18n::{self, gettext};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Zoom {
    Fit,
    Factor(f64),
}

pub const MIN: f64 = 0.05;
pub const MAX: f64 = 20.0;
const STEP: f64 = 1.25;

/// Faktor, mit dem ein Bild `img` in eine Fläche `area` passt (höchstens 1).
pub fn fit_factor(img: (f64, f64), area: (f64, f64)) -> f64 {
    let (iw, ih) = img;
    let (aw, ah) = area;
    if iw <= 0.0 || ih <= 0.0 || aw <= 0.0 || ah <= 0.0 {
        return 1.0;
    }
    (aw / iw).min(ah / ih).min(1.0)
}

/// Nächste Stufe vom aktuell sichtbaren Faktor aus.
pub fn zoom_in(current: f64) -> f64 {
    (current * STEP).clamp(MIN, MAX)
}

pub fn zoom_out(current: f64) -> f64 {
    (current / STEP).clamp(MIN, MAX)
}

/// Zoomstufe für die Statusleiste: „100 %“, „12,5 %“ im Deutschen,
/// „100%“, „12.5%“ im Englischen (Dezimalzeichen und Form aus dem Katalog).
pub fn percent(f: f64) -> String {
    // TRANSLATORS: Zoomstufe in der Statusleiste; {value} ist die Zahl („12.5“)
    percent_with(f, &i18n::decimal_separator(), &gettext("{value}%"))
}

fn percent_with(f: f64, sep: &str, template: &str) -> String {
    let p = f * 100.0;
    let digits = if (p - p.round()).abs() < 0.05 { 0 } else { 1 };
    i18n::fill(template, &[("value", &i18n::decimal(p, digits, sep))])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitting_never_enlarges_and_keeps_the_aspect() {
        assert_eq!(fit_factor((800.0, 600.0), (1600.0, 1200.0)), 1.0);
        assert_eq!(fit_factor((8000.0, 6000.0), (800.0, 600.0)), 0.1);
        assert_eq!(fit_factor((600.0, 900.0), (900.0, 450.0)), 0.5);
        assert_eq!(fit_factor((0.0, 10.0), (100.0, 100.0)), 1.0);
    }

    #[test]
    fn zoom_steps_stay_within_bounds() {
        assert!((zoom_in(1.0) - 1.25).abs() < 1e-9);
        assert!((zoom_out(1.25) - 1.0).abs() < 1e-9);
        assert_eq!(zoom_in(19.0), MAX);
        assert_eq!(zoom_out(0.05), MIN);
    }

    #[test]
    fn percentages_follow_the_catalog() {
        // Deutsch wie bis 0.1.1 (Werte aus de.po)
        assert_eq!(percent_with(1.0, ",", "{value} %"), "100 %");
        assert_eq!(percent_with(0.125, ",", "{value} %"), "12,5 %");
        assert_eq!(percent_with(0.1, ",", "{value} %"), "10 %");
        // ohne Katalog: Englisch
        assert_eq!(percent(0.125), "12.5%");
        assert_eq!(percent(1.0), "100%");
    }
}
