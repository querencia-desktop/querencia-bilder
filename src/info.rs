//! „Bildinformationen“: Datei, Format, Abmessungen, Größe, Änderungsdatum.
//! Ein gewöhnliches Fenster (labwc-Dekoration), Escape schließt.

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

/// Dateigröße deutsch: „2,9 kB“, „1,4 MB“ (Basis 1000 wie GNOME).
pub fn human_size(bytes: u64) -> String {
    let b = bytes as f64;
    let s = if b < 1000.0 {
        format!("{bytes} Bytes")
    } else if b < 1e6 {
        format!("{:.1} kB", b / 1e3)
    } else if b < 1e9 {
        format!("{:.1} MB", b / 1e6)
    } else {
        format!("{:.1} GB", b / 1e9)
    };
    s.replace('.', ",")
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
    rows.push(("Name".into(), name));
    if let Some(dir) = file.parent() {
        rows.push(("Ordner".into(), dir.parse_name().to_string()));
    }
    let path = file.path();
    if let Some((format, w, h)) = path.as_ref().and_then(gdk_pixbuf::Pixbuf::file_info) {
        let name = format.name().map(|n| n.to_uppercase()).unwrap_or_default();
        let desc = format
            .description()
            .map(|d| d.to_string())
            .unwrap_or_default();
        rows.push(("Format".into(), format!("{name} ({desc})")));
        rows.push(("Abmessungen".into(), format!("{w} × {h} Pixel")));
    } else if let Some((w, h)) = size {
        rows.push(("Abmessungen".into(), format!("{w} × {h} Pixel")));
    }
    if let Ok(info) = file.query_info(
        "standard::size,standard::content-type,time::modified",
        gio::FileQueryInfoFlags::NONE,
        gio::Cancellable::NONE,
    ) {
        rows.push(("Dateigröße".into(), human_size(info.size().max(0) as u64)));
        if let Some(t) = info.content_type() {
            rows.push(("Dateityp".into(), t.to_string()));
        }
        if let Some(s) = info
            .modification_date_time()
            .and_then(|d| d.to_local().ok())
            .and_then(|dt| dt.format("%d.%m.%Y %H:%M").ok())
        {
            rows.push(("Geändert".into(), s.to_string()));
        }
    }
    if rotation != 0 {
        rows.push((
            "Anzeige".into(),
            format!("um {rotation}° gedreht (Datei unverändert)"),
        ));
    }
    Facts { rows }
}

thread_local! {
    /// Das (einzige) Fenster „Bildinformationen“, wird wiederverwendet.
    static WINDOW: std::cell::RefCell<Option<gtk::Window>> = const { std::cell::RefCell::new(None) };
}

/// Zeigt die Bildinformationen. Das Fenster wird einmal angelegt und beim
/// Schließen nur verborgen (wie in Eye of GNOME): GTK 4.22.4 stürzt sonst
/// beim Zerstören eines Fensters ab, das aus einem Tastenkürzel heraus
/// geöffnet wurde (Alt+Eingabe; SIGSEGV in libwayland-client, QI-01).
pub fn show(parent: &impl IsA<gtk::Window>, facts: &Facts) {
    let window = WINDOW.with(|slot| {
        slot.borrow_mut()
            .get_or_insert_with(|| build_window(parent.upcast_ref()))
            .clone()
    });
    window.set_child(Some(&content(&window, facts)));
    window.present();
}

fn build_window(parent: &gtk::Window) -> gtk::Window {
    let window = gtk::Window::builder()
        .transient_for(parent)
        .hide_on_close(true)
        .title("Bildinformationen")
        .resizable(false)
        .build();
    window.add_css_class("qb-dialog");
    let keys = gtk::EventControllerKey::new();
    let w = window.downgrade();
    keys.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            if let Some(w) = w.upgrade() {
                w.close();
            }
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    window.add_controller(keys);
    window
}

fn content(window: &gtk::Window, facts: &Facts) -> gtk::Box {
    let grid = gtk::Grid::builder()
        .row_spacing(4)
        .column_spacing(12)
        .build();
    for (i, (k, v)) in facts.rows.iter().enumerate() {
        let key = gtk::Label::new(None);
        key.set_markup(&format!("<b>{}:</b>", glib::markup_escape_text(k)));
        key.set_xalign(1.0);
        key.set_valign(gtk::Align::Start);
        let val = gtk::Label::new(Some(v));
        val.set_xalign(0.0);
        val.set_selectable(true);
        val.set_wrap(true);
        val.set_max_width_chars(50);
        grid.attach(&key, 0, i as i32, 1, 1);
        grid.attach(&val, 1, i as i32, 1, 1);
    }
    let close = gtk::Button::with_mnemonic("_Schließen");
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
            w.close();
        }
    });
    outer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_german_with_decimal_comma() {
        assert_eq!(human_size(19), "19 Bytes");
        assert_eq!(human_size(2920), "2,9 kB");
        assert_eq!(human_size(1_478_886), "1,5 MB");
    }
}
