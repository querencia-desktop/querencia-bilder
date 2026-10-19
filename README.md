# Querencia Bilder 0.1.1

Klassischer Bildbetrachter für Querencia Desktop nach dem Vorbild von
Eye of GNOME 2.30: Menüleiste (Bild | Bearbeiten | Ansicht | Gehe zu |
Hilfe), kompakte Werkzeugleiste, Statuszeile, Vollbild, Diashow.
Rust + GTK 4, Dekodierung über das System-gdk-pixbuf (Ubuntu 26.04: glycin).

> **Status:** 0.1.0 (Prototyp QI-01) läuft seit G2-E in der Sitzung
> (Starter `org.querencia.Bilder.desktop`, ohne MIME-Zuordnung); 0.1.1 ist
> gebaut und wird über die Status-Session freigegeben.
> Querencia Bilder **schreibt nie** in Bilddateien: Drehen wirkt nur auf
> die Anzeige, es gibt keine Speicherfunktion.

| | |
|---|---|
| Programm | `querencia-bilder` |
| Application-ID | `org.querencia.Bilder` (Schema wie `org.querencia.Desktop`/`.Screenshot`; endgültige IDs: Besitzerentscheidung) |
| Symbol | eigenes Motiv, `data/icons/hicolor/scalable/apps/org.querencia.Bilder.svg` (+ PNG 16–256, **nie 48×48**: GTK-4.22-Symbolabsturz); das Info-Fenster nimmt dasselbe SVG aus der eingebetteten Ressource `data/resource/bilder.gresource` |
| Formate | PNG, JPEG, GIF (erstes Bild), BMP, WebP, außerdem alles, was gdk-pixbuf lädt (AVIF, TIFF, …) |
| Tasten | ←/→, Bild↑/↓, Leertaste/Rücktaste: blättern; Pos1/Ende; +/−, Ziffernblock +/−, Strg+Plus/Strg+=/Strg+Minus (zoomen), 1 oder Strg+0 (100 %), F (einpassen); Strg+R / Umschalt+Strg+R drehen; F11 Vollbild; F5 Diashow; Esc beendet Vollbild/Diashow; Alt+Eingabe Bildinformationen (Escape schließt); Strg+O; Strg+W und Strg+Q schließen |

## Bauen und testen

```bash
cargo build --release --locked
cargo test --locked                       # 8 Unit-Tests (Ordnerliste, Zoom, Größen, Ressource)
cargo clippy --all-targets --locked -- -D warnings
```

Die Ressource mit dem Symbol des Info-Fensters wird nach einer Änderung am
SVG neu erzeugt (Befehl in `data/resource/bilder.gresource.xml`); der
Unit-Test `resource::tests` schlägt fehl, solange sie veraltet ist.

GUI-Prüfung nur isoliert (headless labwc, eigener D-Bus, virtuelle Eingabe):
`../werkzeug/` (QI-01, Bericht ) bzw.
`~/projects/querencia-bilder-011/werkzeug/` (0.1.1, Bericht
).

## Bekannte Grenzen

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

