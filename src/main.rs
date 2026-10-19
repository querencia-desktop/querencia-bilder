//! Querencia Bilder – klassischer Bildbetrachter für Querencia Desktop.
//!
//! Vorbild ist Eye of GNOME 2.30 (GNOME 2): Menüleiste, kompakte
//! Werkzeugleiste, Statuszeile, Vollbild und Diashow. GTK 4 ohne libadwaita
//! und ohne eigene Titelleiste: labwc zeichnet die Server-Titelleiste.
//!
//! Querencia Bilder **schreibt nie** in Bilddateien: Drehen wirkt nur auf
//! die Anzeige, es gibt keine Speicherfunktion.

mod area;
mod folder;
mod info;
mod resource;
mod viewer;
mod zoom;

use gtk::prelude::*;
use gtk::{gio, glib};

pub const APP_ID: &str = "org.querencia.Bilder";
pub const APP_NAME: &str = "Querencia Bilder";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    app.connect_startup(|app| {
        gtk::Window::set_default_icon_name(APP_ID);
        viewer::use_classic_icon_theme();
        resource::register();
        viewer::load_css();
        viewer::install_accels(app);
    });
    app.connect_activate(|app| viewer::Viewer::get(app).present());
    app.connect_open(|app, files, _| {
        let v = viewer::Viewer::get(app);
        v.open_files(files);
        v.present();
    });
    app.run()
}
