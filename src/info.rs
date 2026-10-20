//! „Image Properties“ („Bildinformationen“): Datei, Format, Abmessungen,
//! Größe, Änderungsdatum.
//! Ein gewöhnliches Fenster (labwc-Dekoration), Escape schließt.

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

use crate::i18n::{self, gettext, gettext_f, ngettext_f};

/// Dateigröße: „2,9 kB“, „1,4 MB“ im Deutschen, „2.9 kB“ im Englischen
/// (Basis 1000 wie GNOME; Dezimalzeichen aus dem Katalog).
pub fn human_size(bytes: u64) -> String {
    size_with(bytes, &i18n::decimal_separator())
}

fn size_with(bytes: u64, sep: &str) -> String {
    let b = bytes as f64;
    if b < 1000.0 {
        // b < 1000, passt also in u32
        ngettext_f("{n} byte", "{n} bytes", bytes as u32, &[])
    } else if b < 1e6 {
        format!("{} kB", i18n::decimal(b / 1e3, 1, sep))
    } else if b < 1e9 {
        format!("{} MB", i18n::decimal(b / 1e6, 1, sep))
    } else {
        format!("{} GB", i18n::decimal(b / 1e9, 1, sep))
    }
}

/// „800 × 600 pixels“ (Bildinformationen und Statusleiste).
pub fn dimensions(width: i32, height: i32) -> String {
    gettext_f(
        "{width} × {height} pixels",
        &[
            ("width", &width.to_string()),
            ("height", &height.to_string()),
        ],
    )
}

pub struct Facts {
    pub rows: Vec<(String, String)>,
}

/// Sammelt die Angaben zu `file` (Bild bereits geladen: `size`).
pub fn facts(file: &gio::File, size: Option<(i32, i32)>, rotation: i32) -> Facts {
    let mut rows = Vec::new();
    let name = file
        .basename()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    rows.push((gettext("Name"), name));
    if let Some(dir) = file.parent() {
        rows.push((gettext("Folder"), dir.parse_name().to_string()));
    }
    let path = file.path();
    if let Some((format, w, h)) = path.as_ref().and_then(gdk_pixbuf::Pixbuf::file_info) {
        let name = format.name().map(|n| n.to_uppercase()).unwrap_or_default();
        let desc = format
            .description()
            .map(|d| d.to_string())
            .unwrap_or_default();
        rows.push((gettext("Format"), format!("{name} ({desc})")));
        rows.push((gettext("Dimensions"), dimensions(w, h)));
    } else if let Some((w, h)) = size {
        rows.push((gettext("Dimensions"), dimensions(w, h)));
    }
    if let Ok(info) = file.query_info(
        "standard::size,standard::content-type,time::modified",
        gio::FileQueryInfoFlags::NONE,
        gio::Cancellable::NONE,
    ) {
        rows.push((gettext("File Size"), human_size(info.size().max(0) as u64)));
        if let Some(t) = info.content_type() {
            rows.push((gettext("File Type"), t.to_string()));
        }
        if let Some(s) = info
            .modification_date_time()
            .and_then(|d| d.to_local().ok())
            // TRANSLATORS: Datum und Uhrzeit der letzten Änderung, Format von
            // g_date_time_format (%d Tag, %m Monat, %Y Jahr, %H:%M Uhrzeit)
            .and_then(|dt| dt.format(&gettext("%Y-%m-%d %H:%M")).ok())
        {
            rows.push((gettext("Modified"), s.to_string()));
        }
    }
    if rotation != 0 {
        rows.push((
            // TRANSLATORS: Zeile „Anzeige“ in den Bildinformationen, nur bei
            // gedrehter Anzeige
            gettext("Display"),
            gettext_f(
                "rotated by {degrees}° (file unchanged)",
                &[("degrees", &rotation.to_string())],
            ),
        ));
    }
    Facts { rows }
}

/// Werte bis zu dieser Länge stehen in einer Zeile, längere brechen hier um.
const WRAP_CHARS: usize = 50;

thread_local! {
    /// Das offene Fenster „Bildinformationen“ (höchstens eines). Nur schwach
    /// gehalten: Nach dem Schließen gibt GTK es frei, der Platz ist dann leer.
    static WINDOW: glib::WeakRef<gtk::Window> = glib::WeakRef::new();
}

/// Zeigt die Bildinformationen. Ist das Fenster offen, bekommt es nur den
/// neuen Inhalt; sonst wird es neu gebaut. Beim Schließen wird es zerstört,
/// nicht verborgen: Ein verborgenes und wieder gezeigtes GTK-4.22-Fenster hat
/// in labwc 0.9.8 keine Titelleiste mehr (QD-076, Werkstatt QREL-06 A2.4).
pub fn show(parent: &impl IsA<gtk::Window>, facts: &Facts) {
    let window = WINDOW.with(|slot| {
        slot.upgrade().unwrap_or_else(|| {
            let w = dialog_window(parent.upcast_ref(), &gettext("Image Properties"));
            slot.set(Some(&w));
            w
        })
    });
    window.set_child(Some(&content(&window, facts)));
    window.present();
}

/// Ein Nebenfenster von Bilder: gehört zur Anwendung wie das Hauptfenster
/// (App-ID `org.querencia.Bilder` statt des Programmnamens, Muster Querencia
/// Editor), wird mit dem Hauptfenster zerstört und schließt sich erst im
/// nächsten Leerlauf, nicht aus dem laufenden Tasten- oder Klick-Handler.
pub fn dialog_window(parent: &gtk::Window, title: &str) -> gtk::Window {
    let window = gtk::Window::builder()
        .transient_for(parent)
        .destroy_with_parent(true)
        .title(title)
        .resizable(false)
        .build();
    if let Some(app) = parent.application() {
        window.set_application(Some(&app));
    }
    window.add_css_class("qb-dialog");
    let keys = gtk::EventControllerKey::new();
    let w = window.downgrade();
    keys.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            if let Some(w) = w.upgrade() {
                close_later(&w);
            }
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    window.add_controller(keys);
    window
}

/// Schließt (und zerstört) `window` im nächsten Leerlauf.
pub fn close_later(window: &gtk::Window) {
    let window = window.clone();
    glib::idle_add_local_once(move || window.close());
}

fn content(window: &gtk::Window, facts: &Facts) -> gtk::Box {
    let grid = gtk::Grid::builder()
        .row_spacing(4)
        .column_spacing(12)
        .build();
    for (i, (k, v)) in facts.rows.iter().enumerate() {
        let key = gtk::Label::new(None);
        // TRANSLATORS: Bezeichnung einer Zeile in den Bildinformationen, fett
        let k = gettext_f("{label}:", &[("label", k)]);
        key.set_markup(&format!("<b>{}</b>", glib::markup_escape_text(&k)));
        key.set_xalign(1.0);
        key.set_valign(gtk::Align::Start);
        let val = gtk::Label::new(Some(v));
        val.set_xalign(0.0);
        val.set_selectable(true);
        // Nur lange Werte (Ordnerpfad) brechen um, dann mit fester Breite:
        // Ein umbrechendes Label ohne Mindestbreite lässt GTK 4.22 das nicht
        // größenveränderbare Fenster zu niedrig bemessen („Trying to measure
        // GtkWindow … for height of …, but it needs at least …“).
        if v.chars().count() > WRAP_CHARS {
            val.set_wrap(true);
            val.set_width_chars(WRAP_CHARS as i32);
            val.set_max_width_chars(WRAP_CHARS as i32);
        }
        grid.attach(&key, 0, i as i32, 1, 1);
        grid.attach(&val, 1, i as i32, 1, 1);
    }
    let close = gtk::Button::with_mnemonic(&gettext("_Close"));
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    row.set_halign(gtk::Align::End);
    row.set_margin_top(12);
    row.append(&close);
    let outer = gtk::Box::new(gtk::Orientation::Vertical, 0);
    outer.set_margin_top(12);
    outer.set_margin_bottom(12);
    outer.set_margin_start(12);
    outer.set_margin_end(12);
    outer.append(&grid);
    outer.append(&row);
    window.set_default_widget(Some(&close));
    let w = window.downgrade();
    close.connect_clicked(move |_| {
        if let Some(w) = w.upgrade() {
            close_later(&w);
        }
    });
    outer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_use_the_decimal_separator_of_the_catalog() {
        // Deutsch wie bis 0.1.1 (Komma aus de.po)
        assert_eq!(size_with(2920, ","), "2,9 kB");
        assert_eq!(size_with(1_478_886, ","), "1,5 MB");
        // ohne Katalog: Englisch
        assert_eq!(human_size(19), "19 bytes");
        assert_eq!(human_size(1), "1 byte");
        assert_eq!(human_size(2920), "2.9 kB");
    }
}
