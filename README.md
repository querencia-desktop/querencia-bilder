# Querencia Bilder (Prototyp QI-01)

Klassischer Bildbetrachter für Querencia Desktop nach dem Vorbild von
Eye of GNOME 2.30: Menüleiste (Bild | Bearbeiten | Ansicht | Gehe zu |
Hilfe), kompakte Werkzeugleiste, Statuszeile, Vollbild, Diashow.
Rust + GTK 4, Dekodierung über das System-gdk-pixbuf (Ubuntu 26.04: glycin).

> **Status: isolierter Prototyp, nicht installiert, nicht integriert.**
> Querencia Bilder **schreibt nie** in Bilddateien: Drehen wirkt nur auf
> die Anzeige, es gibt keine Speicherfunktion.

| | |
|---|---|
| Programm | `querencia-bilder` |
| Application-ID | `org.querencia.Bilder` (Schema wie `org.querencia.Desktop`/`.Screenshot`; endgültige IDs: Besitzerentscheidung) |
| Symbol | eigenes Motiv, `data/icons/org.querencia.Bilder.svg` (+ PNG 16–256) |
| Formate | PNG, JPEG, GIF (erstes Bild), BMP, WebP, außerdem alles, was gdk-pixbuf lädt (AVIF, TIFF, …) |
| Tasten | ←/→, Bild↑/↓, Leertaste/Rücktaste: blättern; Pos1/Ende; +/−, 1 (100 %), F (einpassen); Strg+R / Umschalt+Strg+R drehen; F11 Vollbild; F5 Diashow; Esc beendet Vollbild/Diashow; Alt+Eingabe Bildinformationen; Strg+O, Strg+W |

## Bauen und testen

```bash
cargo build --release --locked
cargo test --locked                       # 7 Unit-Tests (Ordnerliste, Zoom, Größen)
cargo clippy --all-targets --locked -- -D warnings
```

GUI-Prüfung nur isoliert: `../werkzeug/` (headless labwc, eigener D-Bus,
virtuelle Eingabe), siehe Bericht .

## Bekannte Grenzen

- GIF-Animationen werden als erstes Bild gezeigt.
- Kein EXIF-Detail (Kamera, Belichtung) in den Bildinformationen; die
  EXIF-Ausrichtung wird beim Anzeigen berücksichtigt.
- Keine Miniaturleiste, kein Drucken, kein „In den Papierkorb“.
- Die Desktop-Datei setzt bewusst **keine** `MimeType`-Zeile (siehe Bericht).

Lizenz: GPL-3.0-or-later. Kein Code aus Eye of MATE/Eye of GNOME/Ristretto.
