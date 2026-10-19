# Querencia Bilder 0.2.0

Klassischer Bildbetrachter für Querencia Desktop nach dem Vorbild von
Eye of GNOME 2.30: Menüleiste (Bild | Bearbeiten | Ansicht | Gehe zu |
Hilfe), kompakte Werkzeugleiste, Statuszeile, Vollbild, Diashow.
Rust + GTK 4, Dekodierung über das System-gdk-pixbuf (Ubuntu 26.04: glycin).
Oberfläche englisch und deutsch über GNU gettext (`po/`).

> **Status:** 0.1.1 läuft seit 20.10.2026 in der Sitzung (Starter
> `org.querencia.Bilder.desktop`); 0.2.0 (gettext, Englisch/Deutsch) ist
> gebaut und wird über die Status-Session freigegeben.
> Querencia Bilder **schreibt nie** in Bilddateien: Drehen wirkt nur auf
> die Anzeige, es gibt keine Speicherfunktion.

| | |
|---|---|
| Programm | `querencia-bilder` |
| Application-ID | `org.querencia.Bilder` (Schema wie `org.querencia.Desktop`/`.Screenshot`; endgültige IDs: Besitzerentscheidung) |
| Symbol | eigenes Motiv, `data/icons/hicolor/scalable/apps/org.querencia.Bilder.svg` (+ PNG 16–256, **nie 48×48**: GTK-4.22-Symbolabsturz); das Info-Fenster nimmt dasselbe SVG aus der eingebetteten Ressource `data/resource/bilder.gresource` |
| Sprachen | Englisch (Quelltext, msgid) und Deutsch (`po/de.po`); Wahl über `LANGUAGE`/`LANG`, Kataloge in `<Ablage>/share/locale` oder `/usr/share/locale` |
| Formate | PNG, JPEG, GIF (erstes Bild), BMP, WebP, außerdem alles, was gdk-pixbuf lädt (AVIF, TIFF, …) |
| Tasten | ←/→, Bild↑/↓, Leertaste/Rücktaste: blättern; Pos1/Ende; +/−, Ziffernblock +/−, Strg+Plus/Strg+=/Strg+Minus (zoomen), 1 oder Strg+0 (100 %), F (einpassen); Strg+R / Umschalt+Strg+R drehen; F11 Vollbild; F5 Diashow; Esc beendet Vollbild/Diashow; Alt+Eingabe Bildinformationen (Escape schließt); Strg+O; Strg+W und Strg+Q schließen |

## Bauen und testen

```bash
cargo build --release --locked
cargo test --locked                       # Unit-Tests (Ordnerliste, Zoom, Größen, Ressource, Katalog)
cargo clippy --all-targets --locked -- -D warnings
```

Übersetzungen (ohne Abhängigkeiten; GNU gettext ≥ 0.23 zum Gegenprüfen):

```bash
python3 werkzeug/i18n.py pot        # po/querencia-bilder.pot aus src/ und data/*.desktop.in
python3 werkzeug/i18n.py update     # po/de.po nachziehen, dann übersetzen
python3 werkzeug/i18n.py desktop data/org.querencia.Bilder.desktop.in data/org.querencia.Bilder.desktop
python3 werkzeug/i18n.py pruefen    # muss PASS melden
python3 werkzeug/i18n.py mo <ziel>  # <ziel>/de/LC_MESSAGES/querencia-bilder.mo
msgfmt -c -o /dev/null po/de.po     # Gegenprobe mit GNU gettext
```

Ein Paket legt die `.mo` nach `/usr/share/locale/<lang>/LC_MESSAGES/`; Deutsch
hängt an diesem Katalog (ohne ihn zeigt Bilder Englisch).

Die Ressource mit dem Symbol des Info-Fensters wird nach einer Änderung am
SVG neu erzeugt (Befehl in `data/resource/bilder.gresource.xml`); der
Unit-Test `resource::tests` schlägt fehl, solange sie veraltet ist.

GUI-Prüfung nur isoliert (headless labwc, eigener D-Bus, virtuelle Eingabe):
`../werkzeug/` (QI-01, Bericht ) bzw.
`~/projects/querencia-bilder-011/werkzeug/` (0.1.1, Bericht
) und `~/projects/querencia-bilder-020/werkzeug/`
(0.2.0 je Sprache, Bericht ).

## Bekannte Grenzen

- Programmname und Fenstertitel bleiben „Querencia Bilder“ (nicht übersetzt);
  Fehlermeldungen von gdk-pixbuf und der GTK-Dateiwähler kommen aus deren
  eigenen Katalogen. Diagnoseausgaben auf stderr bleiben deutsch.
- GIF-Animationen werden als erstes Bild gezeigt.
- Kein EXIF-Detail (Kamera, Belichtung) in den Bildinformationen; die
  EXIF-Ausrichtung wird beim Anzeigen berücksichtigt.
- Keine Miniaturleiste, kein Drucken, kein „In den Papierkorb“.
- Die Desktop-Datei setzt bewusst **keine** `MimeType`-Zeile (siehe Bericht).

## Lizenz

Programm: GPL-3.0-or-later (GNU General Public License, Version 3 oder neuer),
siehe [LICENSE](LICENSE). Copyright 2026 Querencia Desktop contributors.
Kein Code aus Eye of MATE/Eye of GNOME/Ristretto; die Rust-Abhängigkeiten
(gtk-rs) stehen unter MIT.

Name und Logo „Querencia“ sind keine freien Inhalte.

