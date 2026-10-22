# Querencia Bilder

Klassischer Bildbetrachter für Querencia Desktop nach dem Vorbild von
Eye of GNOME 2.30: Menüleiste (Bild | Bearbeiten | Ansicht | Gehe zu |
Hilfe), kompakte Werkzeugleiste, Statuszeile, Vollbild, Diashow.
Rust + GTK 4, Dekodierung über das System-gdk-pixbuf (Ubuntu 26.04: glycin).
Oberfläche englisch und deutsch über GNU gettext (`po/`).

> **Alpha.** Querencia Bilder **schreibt nie** in Bilddateien: Drehen wirkt
> nur auf die Anzeige, es gibt keine Speicherfunktion.

## Stand

| | |
|---|---|
| Version | 0.2.6 (Tag `bilder/0.2.6`; Info-Dialog mit Alpha-Hinweis, Website, Copyright und Lizenz, Hilfe-Menü mit Online-Hilfe und Fehler melden) |
| Auf der Entwicklungs-VM | 0.2.5 live seit 21.10.2026; 0.2.6 wartet auf Freigabe |
| Letzte Prüfung | 0.2.6 (22.10.2026): 16/16 Unit-Tests, Clippy sauber, `i18n.py pruefen` PASS; isoliert Deutsch/Englisch 1024×768 mit Browser-Attrappe: Online-Hilfe öffnet /de/docs/ bzw. /en/docs/, Fehler melden die GitHub-Seite (je über die Zugriffstaste), Info-Dialog mit Alpha-Hinweis, Website, Copyright und Lizenz |
| Getestet auf | Ubuntu 26.04, labwc 0.9.8, GTK 4.22 |

Änderungen je Version: [CHANGELOG.md](CHANGELOG.md).

## Zweck

| | |
|---|---|
| Programm | `querencia-bilder` |
| Application-ID | `org.querencia.Bilder` (Schema wie `org.querencia.Desktop`/`.Screenshot`; endgültige IDs: Besitzerentscheidung) |
| Symbol | eigenes Motiv, `data/icons/hicolor/scalable/apps/org.querencia.Bilder.svg` (+ PNG 16–256, **nie 48×48**: GTK-4.22-Symbolabsturz); das Info-Fenster nimmt dasselbe SVG aus der eingebetteten Ressource `data/resource/bilder.gresource` |
| Sprachen | Englisch (Quelltext, msgid) und Deutsch (`po/de.po`); Wahl über `LANGUAGE`/`LANG`, Kataloge in `<Ablage>/share/locale` oder `/usr/share/locale` |
| Formate | PNG, JPEG, GIF (erstes Bild), BMP, WebP, außerdem alles, was gdk-pixbuf lädt (AVIF, TIFF, …) |
| Tasten | ←/→, Bild↑/↓, Leertaste/Rücktaste: blättern; Pos1/Ende; +/−, Ziffernblock +/−, Strg+Plus/Strg+=/Strg+Minus (zoomen), 1 oder Strg+0 (100 %), F (einpassen); Strg+R / Umschalt+Strg+R drehen; F11 Vollbild; F5 Diashow; Esc beendet Vollbild/Diashow; Alt+Eingabe Bildinformationen (Escape schließt); Strg+O; Strg+W und Strg+Q schließen |

## Bauen und Prüfen

`cargo build --release --locked`, danach `cargo fmt --all --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked` und `python3 werkzeug/i18n.py pruefen`.
Übersetzungen, Ressource und die isolierte GUI-Prüfung: [docs/building.md](docs/building.md), [docs/testing.md](docs/testing.md).

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
siehe [LICENSE](LICENSE) (reiner Lizenztext). Copyright 2026 Querencia Desktop
contributors; Urheberhinweis in [COPYRIGHT](COPYRIGHT).
Kein Code aus Eye of MATE/Eye of GNOME/Ristretto; die Rust-Abhängigkeiten
(gtk-rs) stehen unter MIT.

Name und Logo „Querencia“ sind keine freien Inhalte.

