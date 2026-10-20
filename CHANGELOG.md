# Änderungen – Querencia Bilder

## [0.2.2] – 2026-10-20
- Menü: Bilder steht unter „Grafik“ statt unter „Zubehör“
  (`Categories=GTK;Graphics;Viewer;` statt `GTK;Utility;Viewer;`). Die
  Pflichtprobe auf einer frischen VM (B3) zeigte, dass das Menü „Grafik“
  sonst ganz fehlt. Entspricht den Kategorien der MetaInfo (Graphics,
  Viewer). Programm, Texte und Katalog unverändert.

## [0.2.1] – 2026-10-20, CSS-Welle
- Auswahlschrift dunkel (`#1a1a1a` statt `#ffffff`) auf dem orangen Verlauf
  der Menüleiste und der Menüeinträge, passend zum GTK-Thema
  Querencia-Klassisch (Entscheidung 20.10.: dunkle Auswahlschrift). Kontrast
  6,2:1 (unten, `#f07746`) bis 8,5:1 (oben, `#f6a179`), mit Weiß waren es
  2,8:1 bzw. 2,0:1. Verlauf und alles Übrige unverändert; Texte und Katalog
  unverändert.

## [0.2.0] – 2026-10-20, unveröffentlicht
- Mehrsprachig über GNU gettext (Textdomäne `querencia-bilder`, Crate
  `gettext-rs` mit `gettext-system`), Muster Editor 0.2.0 (G2-AC,
  `docs/i18n-muster.md` des Editors): Oberflächentexte im Code englisch im
  Wortschatz von Eye of GNOME 2.30 (Image, Edit, View, Go, Help, Rotate
  Clockwise, Best Fit, Fullscreen, Slideshow, Image Properties …), Deutsch
  aus `po/de.po` mit dem bisherigen Wortlaut. Ohne passenden Katalog
  (Englisch, fr, C) zeigt Bilder Englisch.
- Kataloge unter `<Ablage>/share/locale/<lang>/LC_MESSAGES/querencia-bilder.mo`
  neben der Binary, sonst im Präfix (`/usr/share/locale`);
  `QUERENCIA_BILDER_LOCALEDIR` für Tests. Sprache wie üblich über
  `LANGUAGE`/`LC_ALL`/`LC_MESSAGES`/`LANG`.
- Zahlen in Statusleiste und Bildinformationen mit dem Dezimalzeichen der
  Oberflächensprache aus dem Katalog („12,5 %“, „2,9 kB“; englisch „12.5%“,
  „2.9 kB“). „1 Byte“ statt „1 Bytes“ (Pluralform).
- Desktop-Datei aus der Vorlage `data/org.querencia.Bilder.desktop.in`
  (englisch, `[de]`-Zeilen aus `po/de.po`); Menüname „Image Viewer“ /
  „Bildbetrachter“, Kategorien und MimeType wie live seit G2-E.
- `werkzeug/i18n.py` (aus dem Editor): `.pot`, Abgleich, `.mo`,
  Desktop-Datei, Prüfung; Unit-Test `german_catalog_is_complete`.

## [0.1.1] – 2026-10-19, unveröffentlicht
- Info-Fenster zeigt das neue Programmsymbol als SVG aus der eingebetteten
  Ressource (`data/resource/bilder.gresource`, Symbolname
  `org.querencia.Bilder-info`) statt des alten eingebetteten 64-px-PNG.
- Info-Fenster schließt mit Escape (wie „Bildinformationen“ und Eye of GNOME).
- Zusätzlich Strg+= vergrößern (Eye of GNOME 2.30 / Eye of MATE); +, −,
  Ziffernblock-Plus/-Minus und Strg+Plus/Strg+Minus waren schon belegt.
- Copyright-Zeile „Querencia Desktop contributors“ in README und
  LICENSE-Kopf (Lizenzentscheidung Q5).
- Aus G2-V (Zweig `g2v/symbol`): Programmsymbol flächig neu gezeichnet
  (Rahmen, graue Berge, Sonne in Querencia-Orange, kein Verlauf), Stil der
  übrigen Querencia-Symbole (QD-034); `hicolor/48x48` entfernt (Symbolregel,
  GTK-4.22-Symbolabsturz).

## [0.1.0] – Prototyp QI-01 (2026-10-18), unveröffentlicht
- Bildbetrachter mit Menü-, Werkzeug- und Statusleiste, Ordnernavigation,
  Zoom, Einpassen, Originalgröße, Vollbild, Diashow, Drehen (nur Anzeige),
  Bildinformationen, deutsche Oberfläche, eigenes Symbol.
- Dialoge werden wiederverwendet statt zerstört (Umgehung eines
  GTK-4.22.4-Absturzes beim Zerstören eines per Tastenkürzel geöffneten Fensters).
