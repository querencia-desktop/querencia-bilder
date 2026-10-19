# Änderungen – Querencia Bilder

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
