//! Bildliste: die Bilder des Ordners der geöffneten Datei (wie Eye of GNOME)
//! oder genau die übergebenen Dateien, wenn mehrere geöffnet werden.
//! Reihenfolge wie im Dateimanager: Dateinamen „natürlich“ sortiert
//! (`Bild 2` vor `Bild 10`), Groß-/Kleinschreibung egal.

use std::cmp::Ordering;

/// Endungen, die als Bild gelten (zusätzlich zu `image/*`-Inhaltstypen).
const IMAGE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "jpe", "gif", "bmp", "webp", "tif", "tiff", "ico", "avif", "heic", "jxl",
    "tga", "pnm", "ppm", "pgm", "pbm", "xpm", "svg", "qoi",
];

pub fn looks_like_image(name: &str, content_type: Option<&str>) -> bool {
    if content_type.is_some_and(|t| t.starts_with("image/")) {
        return true;
    }
    name.rsplit_once('.')
        .is_some_and(|(_, ext)| IMAGE_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
}

/// Natürlicher Vergleich: Ziffernfolgen als Zahl, sonst ohne Groß-/Kleinschreibung.
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (mut x, mut y) = (a.chars().peekable(), b.chars().peekable());
    loop {
        match (x.peek().copied(), y.peek().copied()) {
            (None, None) => return a.cmp(b),
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(c), Some(d)) if c.is_ascii_digit() && d.is_ascii_digit() => {
                let mut n = String::new();
                while let Some(c) = x.peek().copied().filter(char::is_ascii_digit) {
                    n.push(c);
                    x.next();
                }
                let mut m = String::new();
                while let Some(d) = y.peek().copied().filter(char::is_ascii_digit) {
                    m.push(d);
                    y.next();
                }
                let (n2, m2) = (n.trim_start_matches('0'), m.trim_start_matches('0'));
                let o = n2.len().cmp(&m2.len()).then_with(|| n2.cmp(m2));
                if o != Ordering::Equal {
                    return o;
                }
            }
            (Some(c), Some(d)) => {
                let o = c.to_lowercase().cmp(d.to_lowercase());
                if o != Ordering::Equal {
                    return o;
                }
                x.next();
                y.next();
            }
        }
    }
}

/// Nächster bzw. voriger Index mit Umlauf am Ende (wie die Diashow).
pub fn step(index: usize, len: usize, forward: bool) -> usize {
    if len == 0 {
        return 0;
    }
    if forward {
        (index + 1) % len
    } else {
        (index + len - 1) % len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_are_recognised_by_type_or_ending() {
        assert!(looks_like_image("a.PNG", None));
        assert!(looks_like_image("foto.webp", None));
        assert!(looks_like_image("ohne-endung", Some("image/jpeg")));
        assert!(!looks_like_image("notiz.txt", Some("text/plain")));
        assert!(!looks_like_image("archiv.tar.gz", None));
    }

    #[test]
    fn names_sort_naturally_and_case_insensitively() {
        let mut v = vec!["Bild 10.png", "bild 2.png", "Bild 1.png", "a.png", "Z.png"];
        v.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(
            v,
            ["a.png", "Bild 1.png", "bild 2.png", "Bild 10.png", "Z.png"]
        );
        // gleiche Zahl: feste Reihenfolge nach dem Text
        assert_eq!(natural_cmp("x007", "x7"), Ordering::Less);
    }

    #[test]
    fn stepping_wraps_around() {
        assert_eq!(step(0, 3, false), 2);
        assert_eq!(step(2, 3, true), 0);
        assert_eq!(step(1, 3, true), 2);
        assert_eq!(step(0, 0, true), 0);
    }
}
