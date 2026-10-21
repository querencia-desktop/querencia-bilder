//! Das Betrachterfenster: Menüleiste, Werkzeugleiste, Bildfläche, Statuszeile.
//!
//! Menüs wie Eye of GNOME 2.30: Image | Edit | View | Go | Help (deutsch
//! Bild | Bearbeiten | Ansicht | Gehe zu | Hilfe aus `po/de.po`).
//! Keine eigene Titelleiste: GTK 4.22 fordert bei labwc die Server-
//! Dekoration an, es gibt also genau eine (die von labwc).
//! Bilder werden im Hintergrund dekodiert; die Oberfläche bleibt bedienbar.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

use crate::area::ImageArea;
use crate::i18n::{gettext, gettext_f, pgettext};
use crate::zoom::{self, Zoom};
use crate::{APP_NAME, folder, info, resource};

const CSS: &str = include_str!("style.css");
/// Anzeigedauer je Bild in der Diashow.
const SLIDESHOW_SECONDS: u32 = 4;

pub fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(CSS);
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// GNOME Human statt Adwaita, wenn der Benutzer GTKs Standard behalten hat
/// (dieselbe Regel wie Panel und Querencia Files).
pub fn use_classic_icon_theme() {
    let Some(settings) = gtk::Settings::default() else {
        return;
    };
    let current = settings
        .gtk_icon_theme_name()
        .map(|s| s.to_string())
        .unwrap_or_default();
    let installed = glib::system_data_dirs()
        .iter()
        .any(|d| d.join("icons/gnome-human/index.theme").is_file());
    if (current.is_empty() || current == "Adwaita") && installed {
        settings.set_gtk_icon_theme_name(Some("gnome-human"));
    }
}

pub fn install_accels(app: &gtk::Application) {
    for (action, keys) in [
        ("win.open", &["<Control>o"][..]),
        ("win.properties", &["<Alt>Return"]),
        ("win.close", &["<Control>w"]),
        ("win.quit", &["<Control>q"]),
        ("win.rotate-right", &["<Control>r"]),
        ("win.rotate-left", &["<Shift><Control>r"]),
        (
            "win.zoom-in",
            &[
                "<Control>plus",
                "<Control>equal",
                "<Control>KP_Add",
                "plus",
                "KP_Add",
            ],
        ),
        (
            "win.zoom-out",
            &[
                "<Control>minus",
                "<Control>KP_Subtract",
                "minus",
                "KP_Subtract",
            ],
        ),
        ("win.zoom-normal", &["<Control>0", "1"]),
        ("win.zoom-fit", &["f"]),
        ("win.fullscreen", &["F11"]),
        ("win.slideshow", &["F5"]),
        ("win.prev", &["Left", "Page_Up", "BackSpace"]),
        ("win.next", &["Right", "Page_Down", "space"]),
        ("win.first", &["Home"]),
        ("win.last", &["End"]),
    ] {
        app.set_accels_for_action(action, keys);
    }
}

fn menu_model() -> gio::Menu {
    fn section(items: &[(String, &str)]) -> gio::Menu {
        let m = gio::Menu::new();
        for (label, action) in items {
            m.append(Some(label), Some(action));
        }
        m
    }
    fn submenu(sections: &[gio::Menu]) -> gio::Menu {
        let m = gio::Menu::new();
        for s in sections {
            m.append_section(None, s);
        }
        m
    }
    // Zugriffstasten (_) je Menü eindeutig, in jeder Sprache für sich.
    let bar = gio::Menu::new();
    bar.append_submenu(
        Some(&gettext("_Image")),
        &submenu(&[
            section(&[(gettext("_Open…"), "win.open")]),
            section(&[(gettext("Prope_rties"), "win.properties")]),
            // TRANSLATORS: Menü „Bild“; Knöpfe in Dialogen nehmen „_Close“ ohne Kontext
            section(&[(pgettext("Image menu", "_Close"), "win.close")]),
        ]),
    );
    bar.append_submenu(
        Some(&gettext("_Edit")),
        &submenu(&[section(&[
            (gettext("_Rotate Clockwise"), "win.rotate-right"),
            (gettext("Rotate Counterc_lockwise"), "win.rotate-left"),
        ])]),
    );
    bar.append_submenu(
        Some(&gettext("_View")),
        &submenu(&[
            section(&[
                (gettext("_Toolbar"), "win.toolbar"),
                (gettext("_Statusbar"), "win.statusbar"),
            ]),
            section(&[
                (gettext("_Zoom In"), "win.zoom-in"),
                (gettext("Zoom _Out"), "win.zoom-out"),
                (gettext("_Normal Size"), "win.zoom-normal"),
                (gettext("_Best Fit"), "win.zoom-fit"),
            ]),
            section(&[
                (gettext("_Fullscreen"), "win.fullscreen"),
                (gettext("S_lideshow"), "win.slideshow"),
            ]),
        ]),
    );
    bar.append_submenu(
        Some(&gettext("_Go")),
        &submenu(&[section(&[
            (gettext("_Previous Image"), "win.prev"),
            (gettext("_Next Image"), "win.next"),
            (gettext("_First Image"), "win.first"),
            (gettext("_Last Image"), "win.last"),
        ])]),
    );
    bar.append_submenu(
        Some(&gettext("_Help")),
        &submenu(&[section(&[(gettext("_About"), "win.about")])]),
    );
    bar
}

fn tool_button(icon: &str, tooltip: &str, action: &str) -> gtk::Button {
    let b = gtk::Button::from_icon_name(icon);
    b.set_tooltip_text(Some(tooltip));
    b.set_action_name(Some(action));
    b.add_css_class("flat");
    b.set_focus_on_click(false);
    // Die Tastatur gehört der Bildfläche (Pfeile, Leertaste blättern).
    b.set_can_focus(false);
    b
}

pub struct Viewer {
    window: gtk::ApplicationWindow,
    toolbar: gtk::Box,
    statusbar: gtk::Box,
    scroller: gtk::ScrolledWindow,
    stack: gtk::Stack,
    area: ImageArea,
    message: gtk::Label,
    status_size: gtk::Label,
    status_zoom: gtk::Label,
    status_index: gtk::Label,
    files: RefCell<Vec<gio::File>>,
    index: Cell<usize>,
    /// Ungedrehtes Bild (Pixbuf) des aktuellen Eintrags; Drehen arbeitet darauf.
    pixbuf: RefCell<Option<gdk_pixbuf::Pixbuf>>,
    rotation: Cell<i32>,
    /// Ladevorgänge zählen: ein veraltetes Ergebnis wird verworfen.
    generation: Cell<u64>,
    slideshow: RefCell<Option<glib::SourceId>>,
    toolbar_visible: Cell<bool>,
    statusbar_visible: Cell<bool>,
}

/// Handler einer Fensteraktion.
type ViewerAction = dyn Fn(&Rc<Viewer>);

thread_local! {
    static VIEWER: RefCell<Option<Rc<Viewer>>> = const { RefCell::new(None) };
}

impl Viewer {
    pub fn get(app: &gtk::Application) -> Rc<Self> {
        if let Some(v) = VIEWER.with(|v| v.borrow().clone()) {
            return v;
        }
        let v = Self::build(app);
        VIEWER.with(|slot| *slot.borrow_mut() = Some(v.clone()));
        v
    }

    fn build(app: &gtk::Application) -> Rc<Self> {
        app.set_menubar(Some(&menu_model()));
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title(APP_NAME)
            .default_width(800)
            .default_height(600)
            .show_menubar(true)
            .build();
        window.add_css_class("qb-window");

        let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        toolbar.add_css_class("qb-toolbar");
        // Kurzhinweise der Werkzeugleiste (Wortlaut wie die Menüeinträge).
        let groups: [Vec<(&str, String, &str)>; 4] = [
            vec![
                ("go-previous", gettext("Previous Image"), "win.prev"),
                ("go-next", gettext("Next Image"), "win.next"),
            ],
            vec![
                ("zoom-in", gettext("Zoom In"), "win.zoom-in"),
                ("zoom-out", gettext("Zoom Out"), "win.zoom-out"),
                ("zoom-original", gettext("Normal Size"), "win.zoom-normal"),
                ("zoom-fit-best", gettext("Best Fit"), "win.zoom-fit"),
            ],
            vec![
                (
                    "object-rotate-left",
                    gettext("Rotate Counterclockwise"),
                    "win.rotate-left",
                ),
                (
                    "object-rotate-right",
                    gettext("Rotate Clockwise"),
                    "win.rotate-right",
                ),
            ],
            vec![
                ("view-fullscreen", gettext("Fullscreen"), "win.fullscreen"),
                (
                    "x-office-presentation",
                    gettext("Slideshow"),
                    "win.slideshow",
                ),
            ],
        ];
        for (i, group) in groups.iter().enumerate() {
            if i > 0 {
                toolbar.append(&gtk::Separator::new(gtk::Orientation::Vertical));
            }
            for (icon, tip, action) in group {
                toolbar.append(&tool_button(icon, tip, action));
            }
        }

        let area = ImageArea::default();
        area.set_hexpand(true);
        area.set_vexpand(true);
        area.set_focusable(true);
        let scroller = gtk::ScrolledWindow::builder().child(&area).build();
        scroller.add_css_class("qb-view");
        let message = gtk::Label::new(None);
        message.set_wrap(true);
        message.set_justify(gtk::Justification::Center);
        message.set_max_width_chars(60);
        let message_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        message_box.set_valign(gtk::Align::Center);
        message_box.set_halign(gtk::Align::Center);
        let warn = gtk::Image::from_icon_name("dialog-warning");
        warn.set_pixel_size(48);
        message_box.append(&warn);
        message_box.append(&message);
        // TRANSLATORS: leeres Fenster; Menüpfad und Tastenkürzel wie in der Oberfläche
        let empty = gtk::Label::new(Some(&gettext("No image loaded.\n\nImage → Open… (Ctrl+O)")));
        empty.set_justify(gtk::Justification::Center);
        empty.add_css_class("qb-leer");
        let stack = gtk::Stack::new();
        stack.add_named(&empty, Some("leer"));
        stack.add_named(&scroller, Some("bild"));
        stack.add_named(&message_box, Some("fehler"));
        stack.set_vexpand(true);
        stack.add_css_class("qb-stack");

        let status_size = gtk::Label::new(None);
        status_size.set_xalign(0.0);
        status_size.set_hexpand(true);
        let status_zoom = gtk::Label::new(None);
        status_zoom.set_width_chars(9);
        let status_index = gtk::Label::new(None);
        status_index.set_width_chars(14);
        let statusbar = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        statusbar.add_css_class("qb-statusbar");
        for w in [&status_size, &status_zoom, &status_index] {
            statusbar.append(w);
        }

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.append(&toolbar);
        vbox.append(&stack);
        vbox.append(&statusbar);
        window.set_child(Some(&vbox));

        let v = Rc::new(Self {
            window,
            toolbar,
            statusbar,
            scroller,
            stack,
            area,
            message,
            status_size,
            status_zoom,
            status_index,
            files: RefCell::new(Vec::new()),
            index: Cell::new(0),
            pixbuf: RefCell::new(None),
            rotation: Cell::new(0),
            generation: Cell::new(0),
            slideshow: RefCell::new(None),
            toolbar_visible: Cell::new(true),
            statusbar_visible: Cell::new(true),
        });
        v.install_actions();
        v.apply_zoom(Zoom::Fit);

        // Größe ändert sich (Fenster, Vollbild): eingepasster Faktor neu anzeigen.
        let weak = Rc::downgrade(&v);
        v.area.add_tick_callback(move |area, _| {
            if let Some(v) = weak.upgrade() {
                let text = if area.texture().is_some() {
                    zoom::percent(area.effective_factor())
                } else {
                    String::new()
                };
                if v.status_zoom.text() != text {
                    v.status_zoom.set_text(&text);
                }
            }
            glib::ControlFlow::Continue
        });
        // Strg+Mausrad zoomt wie in EOG.
        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        let weak = Rc::downgrade(&v);
        scroll.connect_scroll(move |c, _, dy| {
            let Some(v) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            if !c
                .current_event_state()
                .contains(gdk::ModifierType::CONTROL_MASK)
            {
                return glib::Propagation::Proceed;
            }
            let f = v.area.effective_factor();
            v.apply_zoom(Zoom::Factor(if dy < 0.0 {
                zoom::zoom_in(f)
            } else {
                zoom::zoom_out(f)
            }));
            glib::Propagation::Stop
        });
        v.scroller.add_controller(scroll);
        // Betrachtertasten ohne Modifikator fängt das Fenster in der Capture-
        // Phase ab: sonst verbraucht GTKs Fokusnavigation Pfeile und Leertaste,
        // bevor die Anwendungskürzel greifen. Escape beendet Diashow/Vollbild.
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(&v);
        keys.connect_key_pressed(move |_, key, _, state| {
            let Some(v) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            let mods = state
                & (gdk::ModifierType::CONTROL_MASK
                    | gdk::ModifierType::ALT_MASK
                    | gdk::ModifierType::SUPER_MASK);
            if !mods.is_empty() {
                return glib::Propagation::Proceed;
            }
            if key == gdk::Key::Escape {
                if v.window.is_fullscreen() {
                    v.leave_fullscreen();
                    return glib::Propagation::Stop;
                }
                return glib::Propagation::Proceed;
            }
            match viewer_key_action(key) {
                Some(action) => {
                    WidgetExt::activate_action(&v.window, &format!("win.{action}"), None).ok();
                    glib::Propagation::Stop
                }
                None => glib::Propagation::Proceed,
            }
        });
        v.window.add_controller(keys);
        v.window.connect_close_request(|_| {
            if std::env::var_os("QUERENCIA_BILDER_DEBUG").is_some() {
                eprintln!("querencia-bilder: Hauptfenster wird geschlossen");
            }
            VIEWER.with(|slot| *slot.borrow_mut() = None);
            glib::Propagation::Proceed
        });
        v.update_status();
        v
    }

    pub fn present(&self) {
        self.window.present();
        self.area.grab_focus();
    }

    /// Eine Datei: ihr Ordner wird die Bildliste. Mehrere: genau diese.
    pub fn open_files(self: &Rc<Self>, files: &[gio::File]) {
        let Some(first) = files.first() else { return };
        let list = if files.len() > 1 {
            files.to_vec()
        } else {
            folder_images(first).unwrap_or_else(|| vec![first.clone()])
        };
        let index = list.iter().position(|f| f.equal(first)).unwrap_or(0);
        *self.files.borrow_mut() = list;
        self.show_index(index);
    }

    fn current_file(&self) -> Option<gio::File> {
        self.files.borrow().get(self.index.get()).cloned()
    }

    fn show_index(self: &Rc<Self>, index: usize) {
        let Some(file) = self.files.borrow().get(index).cloned() else {
            return;
        };
        self.index.set(index);
        self.rotation.set(0);
        self.pixbuf.replace(None);
        let generation = self.generation.get() + 1;
        self.generation.set(generation);
        let name = display_name(&file);
        self.window.set_title(Some(&format!("{name} – {APP_NAME}")));
        self.status_size.set_text(&gettext("Loading…"));
        self.update_index_label();

        let path = file.path();
        let weak = Rc::downgrade(self);
        let started = std::time::Instant::now();
        glib::spawn_future_local(async move {
            // Dekodieren im Hintergrund; zurück kommen nur Pixeldaten (Pixbuf
            // selbst ist nicht Send) oder der Fehlertext.
            let decoded = match path {
                Some(p) => gio::spawn_blocking(move || decode(&p))
                    .await
                    .unwrap_or_else(|_| Err(gettext("Loading cancelled"))),
                None => Err(gettext("Only local files are supported.")),
            };
            let result = decoded.and_then(|d| d.into_pixbuf());
            // Diagnose ohne Bildinhalte: Name, Dauer, Ergebnis.
            if std::env::var_os("QUERENCIA_BILDER_DEBUG").is_some() {
                match &result {
                    Ok(pb) => eprintln!(
                        "querencia-bilder: geladen {name} {}x{} in {} ms",
                        pb.width(),
                        pb.height(),
                        started.elapsed().as_millis()
                    ),
                    Err(e) => eprintln!("querencia-bilder: Fehler {name}: {e}"),
                }
            }
            let Some(v) = weak.upgrade() else { return };
            if v.generation.get() != generation {
                return; // inzwischen ein anderes Bild gewählt
            }
            match result {
                Ok(pb) => {
                    v.pixbuf.replace(Some(pb));
                    v.render();
                    v.stack.set_visible_child_name("bild");
                }
                Err(err) => {
                    v.area.set_texture(None);
                    let title = gettext_f("Could not load image “{name}”.", &[("name", &name)]);
                    v.message.set_markup(&format!(
                        "<b>{}</b>\n\n{}",
                        glib::markup_escape_text(&title),
                        glib::markup_escape_text(&err)
                    ));
                    v.stack.set_visible_child_name("fehler");
                }
            }
            v.update_status();
        });
    }

    /// Aktuelles Pixbuf mit Drehung als Textur anzeigen.
    fn render(&self) {
        let Some(pb) = self.pixbuf.borrow().clone() else {
            return;
        };
        let rotated = match self.rotation.get().rem_euclid(360) {
            90 => pb.rotate_simple(gdk_pixbuf::PixbufRotation::Clockwise),
            180 => pb.rotate_simple(gdk_pixbuf::PixbufRotation::Upsidedown),
            270 => pb.rotate_simple(gdk_pixbuf::PixbufRotation::Counterclockwise),
            _ => Some(pb),
        };
        if let Some(pb) = rotated {
            self.area.set_texture(Some(&texture_of(&pb)));
        }
    }

    fn update_index_label(&self) {
        let n = self.files.borrow().len();
        self.status_index.set_text(&if n == 0 {
            String::new()
        } else {
            // TRANSLATORS: Statusleiste, Position in der Bildliste
            gettext_f(
                "Image {index} of {count}",
                &[
                    ("index", &(self.index.get() + 1).to_string()),
                    ("count", &n.to_string()),
                ],
            )
        });
    }

    fn update_status(&self) {
        self.update_index_label();
        let Some(pb) = self.pixbuf.borrow().clone() else {
            if self.files.borrow().is_empty() {
                self.status_size.set_text("");
                self.stack.set_visible_child_name("leer");
            } else if self.stack.visible_child_name().as_deref() == Some("fehler") {
                // TRANSLATORS: Statusleiste, Bild konnte nicht geladen werden
                self.status_size.set_text(&gettext("Not readable"));
            }
            return;
        };
        let size = self
            .current_file()
            .and_then(|f| {
                f.query_info(
                    "standard::size",
                    gio::FileQueryInfoFlags::NONE,
                    gio::Cancellable::NONE,
                )
                .ok()
            })
            .map(|i| info::human_size(i.size().max(0) as u64))
            .unwrap_or_default();
        let dims = info::dimensions(pb.width(), pb.height());
        let args = [("dimensions", dims.as_str()), ("size", size.as_str())];
        self.status_size
            .set_text(&if self.rotation.get().rem_euclid(360) != 0 {
                // TRANSLATORS: Statusleiste; die drei Leerzeichen trennen die Felder
                gettext_f("{dimensions}   {size}   rotated (not saved)", &args)
            } else {
                // TRANSLATORS: Statusleiste; die drei Leerzeichen trennen die Felder
                gettext_f("{dimensions}   {size}", &args)
            });
    }

    fn apply_zoom(&self, z: Zoom) {
        let policy = if matches!(z, Zoom::Fit) {
            gtk::PolicyType::Never
        } else {
            gtk::PolicyType::Automatic
        };
        self.scroller.set_policy(policy, policy);
        self.area.set_zoom(z);
    }

    fn step(self: &Rc<Self>, forward: bool) {
        let n = self.files.borrow().len();
        if n > 0 {
            self.show_index(folder::step(self.index.get(), n, forward));
        }
    }

    fn enter_fullscreen(&self) {
        self.window.set_show_menubar(false);
        self.toolbar.set_visible(false);
        self.statusbar.set_visible(false);
        self.window.add_css_class("qb-fullscreen");
        self.apply_zoom(Zoom::Fit);
        self.window.fullscreen();
    }

    fn leave_fullscreen(&self) {
        self.stop_slideshow();
        self.window.unfullscreen();
        self.window.remove_css_class("qb-fullscreen");
        self.window.set_show_menubar(true);
        self.toolbar.set_visible(self.toolbar_visible.get());
        self.statusbar.set_visible(self.statusbar_visible.get());
    }

    fn stop_slideshow(&self) {
        if let Some(id) = self.slideshow.borrow_mut().take() {
            id.remove();
        }
    }

    fn start_slideshow(self: &Rc<Self>) {
        if self.files.borrow().is_empty() {
            return;
        }
        self.enter_fullscreen();
        let weak = Rc::downgrade(self);
        let id = glib::timeout_add_seconds_local(SLIDESHOW_SECONDS, move || {
            let Some(v) = weak.upgrade() else {
                return glib::ControlFlow::Break;
            };
            v.step(true);
            glib::ControlFlow::Continue
        });
        self.slideshow.replace(Some(id));
    }

    fn install_actions(self: &Rc<Self>) {
        let add = |name: &str, f: Box<ViewerAction>| {
            let a = gio::SimpleAction::new(name, None);
            let weak = Rc::downgrade(self);
            a.connect_activate(move |_, _| {
                if let Some(v) = weak.upgrade() {
                    f(&v);
                }
            });
            self.window.add_action(&a);
        };
        add(
            "open",
            Box::new(|v| {
                let filter = gtk::FileFilter::new();
                filter.set_name(Some(&gettext("Images")));
                filter.add_pixbuf_formats();
                let filters = gio::ListStore::new::<gtk::FileFilter>();
                filters.append(&filter);
                let dialog = gtk::FileDialog::builder()
                    .title(gettext("Open Image"))
                    .modal(true)
                    .filters(&filters)
                    .build();
                if let Some(dir) = v.current_file().and_then(|f| f.parent()) {
                    dialog.set_initial_folder(Some(&dir));
                }
                let weak = Rc::downgrade(v);
                dialog.open_multiple(Some(&v.window), gio::Cancellable::NONE, move |r| {
                    let (Some(v), Ok(model)) = (weak.upgrade(), r) else {
                        return;
                    };
                    let files: Vec<gio::File> = (0..model.n_items())
                        .filter_map(|i| model.item(i).and_downcast())
                        .collect();
                    v.open_files(&files);
                });
            }),
        );
        add(
            "properties",
            Box::new(|v| {
                if let Some(f) = v.current_file() {
                    let size = v.pixbuf.borrow().as_ref().map(|p| (p.width(), p.height()));
                    info::show(
                        &v.window,
                        &info::facts(&f, size, v.rotation.get().rem_euclid(360)),
                    );
                }
            }),
        );
        add("close", Box::new(|v| v.window.close()));
        add("quit", Box::new(|v| v.window.close()));
        for (name, delta) in [("rotate-right", 90), ("rotate-left", -90)] {
            add(
                name,
                Box::new(move |v| {
                    if v.pixbuf.borrow().is_some() {
                        v.rotation.set((v.rotation.get() + delta).rem_euclid(360));
                        v.render();
                        v.update_status();
                    }
                }),
            );
        }
        add(
            "zoom-in",
            Box::new(|v| v.apply_zoom(Zoom::Factor(zoom::zoom_in(v.area.effective_factor())))),
        );
        add(
            "zoom-out",
            Box::new(|v| v.apply_zoom(Zoom::Factor(zoom::zoom_out(v.area.effective_factor())))),
        );
        add("zoom-normal", Box::new(|v| v.apply_zoom(Zoom::Factor(1.0))));
        add("zoom-fit", Box::new(|v| v.apply_zoom(Zoom::Fit)));
        add(
            "fullscreen",
            Box::new(|v| {
                if v.window.is_fullscreen() {
                    v.leave_fullscreen();
                } else {
                    v.enter_fullscreen();
                }
            }),
        );
        add(
            "slideshow",
            Box::new(|v| {
                if v.slideshow.borrow().is_some() {
                    v.leave_fullscreen();
                } else {
                    v.start_slideshow();
                }
            }),
        );
        add("prev", Box::new(|v| v.step(false)));
        add("next", Box::new(|v| v.step(true)));
        add(
            "first",
            Box::new(|v| {
                if !v.files.borrow().is_empty() {
                    v.show_index(0);
                }
            }),
        );
        add(
            "last",
            Box::new(|v| {
                let n = v.files.borrow().len();
                if n > 0 {
                    v.show_index(n - 1);
                }
            }),
        );
        add("about", Box::new(|v| about(&v.window)));

        for (name, which) in [("toolbar", 0), ("statusbar", 1)] {
            let a = gio::SimpleAction::new_stateful(name, None, &true.to_variant());
            let weak = Rc::downgrade(self);
            a.connect_activate(move |a, _| {
                let Some(v) = weak.upgrade() else { return };
                let on = !a.state().and_then(|s| s.get::<bool>()).unwrap_or(true);
                a.set_state(&on.to_variant());
                if which == 0 {
                    v.toolbar_visible.set(on);
                    v.toolbar.set_visible(on);
                } else {
                    v.statusbar_visible.set(on);
                    v.statusbar.set_visible(on);
                }
            });
            self.window.add_action(&a);
        }
    }
}

/// Taste ohne Modifikator → Fensteraktion (wie Eye of GNOME).
fn viewer_key_action(key: gdk::Key) -> Option<&'static str> {
    use gdk::Key;
    Some(match key {
        Key::Right | Key::Page_Down | Key::space | Key::KP_Right | Key::KP_Page_Down => "next",
        Key::Left | Key::Page_Up | Key::BackSpace | Key::KP_Left | Key::KP_Page_Up => "prev",
        Key::Home | Key::KP_Home => "first",
        Key::End | Key::KP_End => "last",
        Key::plus | Key::KP_Add | Key::equal => "zoom-in",
        Key::minus | Key::KP_Subtract => "zoom-out",
        Key::_1 | Key::KP_1 => "zoom-normal",
        Key::f | Key::F => "zoom-fit",
        Key::F11 => "fullscreen",
        Key::F5 => "slideshow",
        _ => return None,
    })
}

fn display_name(file: &gio::File) -> String {
    file.basename()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| file.uri().to_string())
}

/// Bilder im Ordner von `file`, natürlich sortiert; `None`, wenn der Ordner
/// nicht lesbar ist.
fn folder_images(file: &gio::File) -> Option<Vec<gio::File>> {
    let dir = file.parent()?;
    let e = dir
        .enumerate_children(
            "standard::name,standard::type,standard::content-type,standard::is-hidden",
            gio::FileQueryInfoFlags::NONE,
            gio::Cancellable::NONE,
        )
        .ok()?;
    let mut names: Vec<String> = Vec::new();
    while let Ok(Some(info)) = e.next_file(gio::Cancellable::NONE) {
        if info.file_type() != gio::FileType::Regular || info.is_hidden() {
            continue;
        }
        let name = info.name().to_string_lossy().into_owned();
        if folder::looks_like_image(&name, info.content_type().as_deref()) {
            names.push(name);
        }
    }
    names.sort_by(|a, b| folder::natural_cmp(a, b));
    let mut list: Vec<gio::File> = names.iter().map(|n| dir.child(n)).collect();
    // Die geöffnete Datei gehört immer dazu (auch mit unbekannter Endung).
    if !list.iter().any(|f| f.equal(file)) {
        list.push(file.clone());
    }
    Some(list)
}

/// Dekodiertes Bild als Pixeldaten (thread-übergreifend übertragbar).
struct Decoded {
    bytes: glib::Bytes,
    width: i32,
    height: i32,
    rowstride: i32,
    alpha: bool,
}

impl Decoded {
    fn into_pixbuf(self) -> Result<gdk_pixbuf::Pixbuf, String> {
        Ok(gdk_pixbuf::Pixbuf::from_bytes(
            &self.bytes,
            gdk_pixbuf::Colorspace::Rgb,
            self.alpha,
            8,
            self.width,
            self.height,
            self.rowstride,
        ))
    }
}

/// Lädt und dreht nach EXIF-Ausrichtung (nur die Anzeige, nie die Datei).
fn decode(path: &std::path::Path) -> Result<Decoded, String> {
    let debug = std::env::var_os("QUERENCIA_BILDER_DEBUG").is_some();
    if debug {
        eprintln!("querencia-bilder: dekodiere {}", path.display());
    }
    let pb = gdk_pixbuf::Pixbuf::from_file(path).map_err(|e| e.message().to_string())?;
    if debug {
        eprintln!("querencia-bilder: dekodiert {}x{}", pb.width(), pb.height());
    }
    let pb = pb.apply_embedded_orientation().unwrap_or(pb);
    Ok(Decoded {
        bytes: pb.read_pixel_bytes(),
        width: pb.width(),
        height: pb.height(),
        rowstride: pb.rowstride(),
        alpha: pb.has_alpha(),
    })
}

fn texture_of(pb: &gdk_pixbuf::Pixbuf) -> gdk::Texture {
    let format = if pb.has_alpha() {
        gdk::MemoryFormat::R8g8b8a8
    } else {
        gdk::MemoryFormat::R8g8b8
    };
    gdk::MemoryTexture::new(
        pb.width(),
        pb.height(),
        format,
        &pb.read_pixel_bytes(),
        pb.rowstride() as usize,
    )
    .upcast()
}

thread_local! {
    /// Das offene Fenster „Info“ (höchstens eines), schwach gehalten wie in
    /// info::show.
    static ABOUT: glib::WeakRef<gtk::Window> = glib::WeakRef::new();
}

/// „About“ („Info“): wird bei jedem Öffnen neu gebaut und beim Schließen
/// zerstört (QD-076, siehe info::show); Escape schließt wie der Knopf.
fn about(parent: &gtk::ApplicationWindow) {
    if let Some(w) = ABOUT.with(|a| a.upgrade()) {
        w.present();
        return;
    }
    let window = info::dialog_window(
        parent.upcast_ref(),
        &gettext_f("About {name}", &[("name", APP_NAME)]),
    );
    window.set_modal(true);
    let body = gtk::Box::new(gtk::Orientation::Vertical, 8);
    body.set_margin_top(18);
    body.set_margin_bottom(12);
    body.set_margin_start(24);
    body.set_margin_end(24);
    // Programmsymbol als SVG aus der eingebetteten Ressource (resource.rs).
    let img = gtk::Image::from_icon_name(resource::INFO_ICON);
    img.set_pixel_size(64);
    body.append(&img);
    let name = gtk::Label::new(None);
    name.set_markup(&format!(
        "<b><big>{APP_NAME} {}</big></b>",
        env!("CARGO_PKG_VERSION")
    ));
    let text = gtk::Label::new(Some(&gettext(
        "Classic image viewer for Querencia Desktop\nmodelled on Eye of GNOME 2.30.\n\n\
         Images are only displayed, never changed.\nPrototype – not released for everyday use.",
    )));
    text.set_justify(gtk::Justification::Center);
    let lic = gtk::Label::new(Some(&gettext("License: GNU GPL, version 3 or later")));
    lic.add_css_class("dim-label");
    let close = gtk::Button::with_mnemonic(&gettext("_Close"));
    close.set_halign(gtk::Align::End);
    close.set_margin_top(12);
    for w in [
        name.upcast_ref::<gtk::Widget>(),
        text.upcast_ref(),
        lic.upcast_ref(),
        close.upcast_ref(),
    ] {
        body.append(w);
    }
    window.set_child(Some(&body));
    window.set_default_widget(Some(&close));
    let w = window.downgrade();
    close.connect_clicked(move |_| {
        if let Some(w) = w.upgrade() {
            info::close_later(&w);
        }
    });
    ABOUT.with(|a| a.set(Some(&window)));
    window.present();
}

#[cfg(test)]
mod tests {
    use super::CSS;

    #[test]
    fn css_repeats_no_light_theme_colors() {
        // QD-081: Grund, Schrift und Ränder kommen aus dem Thema, damit
        // „Dunkle Darstellung bevorzugen“ die Leisten und Menüs mitnimmt.
        for farbe in ["#f2f1f0", "#f5f4f2", "#c7c2be", "#3c3c3c"] {
            assert!(!CSS.contains(farbe), "feste Hellfarbe {farbe} in style.css");
        }
        assert_eq!(CSS.matches("solid @borders;").count(), 2);
    }

    /// Relative Leuchtdichte nach WCAG 2.x für "#rrggbb".
    fn leucht(farbe: &str) -> f64 {
        let kanal = |i: usize| {
            let c = f64::from(u8::from_str_radix(&farbe[i..i + 2], 16).unwrap()) / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * kanal(1) + 0.7152 * kanal(3) + 0.0722 * kanal(5)
    }

    fn farbe_nach(selektor: &str, eigenschaft: &str) -> String {
        let block = CSS.split(selektor).nth(1).expect(selektor);
        let wert = block.split(eigenschaft).nth(1).expect(eigenschaft);
        wert.trim_start_matches(':').trim()[..7].to_string()
    }

    #[test]
    fn empty_hint_reaches_aa_on_image_area() {
        // QD-089: Leerhinweis auf der Bildfläche (grau, im Vollbild schwarz)
        // in beiden Farbschemata mindestens 4,5:1.
        let paare = [
            (
                "stack.qb-stack, scrolledwindow.qb-view, qb-image {",
                "stack.qb-stack > label.qb-leer {",
            ),
            (
                "window.qb-fullscreen qb-image {",
                "window.qb-fullscreen stack.qb-stack > label.qb-leer {",
            ),
        ];
        for (grund, schrift) in paare {
            let g = leucht(&farbe_nach(grund, "background-color"));
            let s = leucht(&farbe_nach(schrift, "color"));
            let k = (g.max(s) + 0.05) / (g.min(s) + 0.05);
            assert!(k >= 4.5, "{schrift} nur {k:.2}:1");
        }
    }
}
