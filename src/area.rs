//! Bildfläche: zeichnet eine Textur zentriert, eingepasst oder mit festem
//! Faktor. Im Faktor-Modus meldet sie die skalierte Größe, damit das
//! umgebende `GtkScrolledWindow` Rollbalken zeigt; eingepasst ist sie so
//! groß wie der verfügbare Platz.

use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene, gsk};

use crate::zoom::{self, Zoom};

mod imp {
    use super::*;

    pub struct ImageArea {
        pub texture: RefCell<Option<gdk::Texture>>,
        pub zoom: Cell<Zoom>,
    }

    impl Default for ImageArea {
        fn default() -> Self {
            Self {
                texture: RefCell::new(None),
                zoom: Cell::new(Zoom::Fit),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImageArea {
        const NAME: &'static str = "QuerenciaBilderImageArea";
        type Type = super::ImageArea;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("qb-image");
        }
    }

    impl ObjectImpl for ImageArea {}

    impl WidgetImpl for ImageArea {
        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            let size = match (self.texture.borrow().as_ref(), self.zoom.get()) {
                (Some(t), Zoom::Factor(f)) => {
                    let v = if orientation == gtk::Orientation::Horizontal {
                        t.width()
                    } else {
                        t.height()
                    };
                    (f64::from(v) * f).ceil() as i32
                }
                _ => 0,
            };
            (size, size, -1, -1)
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let Some(texture) = self.texture.borrow().clone() else {
                return;
            };
            let widget = self.obj();
            let (w, h) = (f64::from(widget.width()), f64::from(widget.height()));
            let (tw, th) = (f64::from(texture.width()), f64::from(texture.height()));
            let f = self.obj().effective_factor();
            let (dw, dh) = (tw * f, th * f);
            let x = ((w - dw) / 2.0).max(0.0);
            let y = ((h - dh) / 2.0).max(0.0);
            let rect = graphene::Rect::new(x as f32, y as f32, dw as f32, dh as f32);
            // Verkleinert: weich (Trilinear); vergrößert: Pixel sichtbar wie in EOG.
            let filter = if f < 1.0 {
                gsk::ScalingFilter::Trilinear
            } else if f > 1.0 {
                gsk::ScalingFilter::Nearest
            } else {
                gsk::ScalingFilter::Linear
            };
            snapshot.append_scaled_texture(&texture, filter, &rect);
        }
    }
}

glib::wrapper! {
    pub struct ImageArea(ObjectSubclass<imp::ImageArea>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ImageArea {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ImageArea {
    pub fn set_texture(&self, texture: Option<&gdk::Texture>) {
        self.imp().texture.replace(texture.cloned());
        self.queue_resize();
    }

    pub fn texture(&self) -> Option<gdk::Texture> {
        self.imp().texture.borrow().clone()
    }

    pub fn zoom(&self) -> Zoom {
        self.imp().zoom.get()
    }

    pub fn set_zoom(&self, zoom: Zoom) {
        self.imp().zoom.set(zoom);
        self.queue_resize();
    }

    /// Tatsächlicher Faktor (eingepasst: abhängig von der Flächengröße).
    pub fn effective_factor(&self) -> f64 {
        let Some(t) = self.imp().texture.borrow().clone() else {
            return 1.0;
        };
        match self.imp().zoom.get() {
            Zoom::Factor(f) => f,
            Zoom::Fit => zoom::fit_factor(
                (f64::from(t.width()), f64::from(t.height())),
                (
                    f64::from(self.width().max(1)),
                    f64::from(self.height().max(1)),
                ),
            ),
        }
    }
}
