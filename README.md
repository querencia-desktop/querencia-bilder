# Querencia Bilder

Klassischer Bildbetrachter für alle, die Querencia Desktop benutzen. Teil von
[Querencia Desktop](https://querencialinux.org) – einer
klassischen Desktopumgebung im GNOME-2-Stil für Wayland (labwc, Rust, GTK 4).
Installiert wird es zusammen mit Querencia Desktop aus der APT-Quelle;
Anleitung auf <https://querencialinux.org>.

*English summary:* A classic image viewer modelled on GNOME 2's Eye of GNOME
2.30; it never writes to image files. Part of Querencia Desktop, a classic
GNOME-2-style desktop for Wayland. Alpha software; documentation is in German,
the user interface is available in English and German.

> **Alpha.** Keine Miniaturleiste, kein Drucken, keine Animationen. Keine
> Sicherheits- oder Datenverlustgarantie.

## Zweck

- Bilder ansehen mit Menüleiste (Bild · Bearbeiten · Ansicht · Gehe zu ·
  Hilfe), kompakter Werkzeugleiste und Statuszeile; Blättern im Ordner,
  Zoomen, Einpassen, Drehen (nur Anzeige), Vollbild (F11), Diashow (F5),
  Bildinformationen (Alt+Eingabe).
- Formate: PNG, JPEG, GIF (erstes Bild), BMP, WebP und alles, was das
  System-gdk-pixbuf lädt (Ubuntu 26.04: glycin-Lader, z. B. AVIF, TIFF).
- Oberfläche auf Englisch, Deutsch, Italienisch, Spanisch und Französisch
  über GNU gettext (englische msgid, `po/<sprache>.po`); Auswahl über
  `LANGUAGE`/`LANG`.
- Bewusst **nicht** Teil dieses Programms: Bearbeiten und Speichern
  (Querencia Bilder schreibt nie in Bilddateien), Miniaturleiste, Drucken,
  „In den Papierkorb“, EXIF-Details.

Anwendungskennung (App-ID): `org.querencia.Bilder` · Binary:
`querencia-bilder` · Debian-Paket: `querencia-bilder` (Paketierung in `querencia-packaging`)

| Datei | Inhalt |
|---|---|
| `src/` | Fenster, Ansicht, Ordnerliste, Zoom, Bildinformationen, Ressource, Übersetzung (`i18n.rs`) |
| `po/` | Vorlage `querencia-bilder.pot`, Übersetzungen `de.po`, `it.po`, `es.po`, `fr.po` |
| `data/` | Desktop-Datei (aus `.desktop.in`), Programmsymbol (PNG 16–256 und SVG, ohne 48×48), eingebettete Ressource |
| `werkzeug/i18n.py` | Kataloge prüfen und erzeugen ohne GNU gettext |

## Stand

| | |
|---|---|
| Version | 0.2.7 (Tag `bilder/0.2.7`; Italienisch, Spanisch, Französisch); seit 0.2.6 Info-Dialog mit Alpha-Hinweis, Website, Copyright und Lizenz, Hilfe-Menü mit Online-Hilfe und „Fehler melden“); seit 0.2.4 dunkle Darstellung, seit 0.2.5 lesbarer Leerhinweis |
| Letzte Prüfung | 26.10.2026 (0.2.7): 16 Unit-Tests, Clippy sauber, `i18n.py pruefen` und `msgfmt --check` für de/es/fr/it; isolierte GUI-Prüfung de/en/it/es/fr bei 1024×768: alle Menüs, Bildinformationen, Online-Hilfe, „Fehler melden“, Info-Dialog (je 12 PASS). 22.10.2026 (0.2.6): 16 Unit-Tests, Clippy sauber, `i18n.py pruefen`; isolierte GUI-Prüfung Deutsch und Englisch bei 1024×768: Online-Hilfe, „Fehler melden“ (je über die Zugriffstaste) und Info-Dialog. 21.10.2026 (0.2.5): isoliert Deutsch und Englisch bei 1024×768 und 1920×1080, hell und dunkel |
| Bekannte Lücken | GIF nur erstes Bild; Programmname und Fenstertitel bleiben „Querencia Bilder“; Meldungen von gdk-pixbuf und dem GTK-Dateiwähler kommen aus deren Katalogen; Diagnoseausgaben auf stderr deutsch; die isolierte GUI-Prüfung liegt noch nicht in diesem Repo |
| Getestet auf | Ubuntu 26.04, labwc 0.9.8, GTK 4.22, Rust 1.93 |
| Nicht getestet | andere Distributionen, HiDPI, Mehrmonitor |

Änderungen: [CHANGELOG.md](CHANGELOG.md).

## Bauen

Voraussetzungen (Ubuntu 26.04):

```bash
sudo apt install build-essential pkg-config rustc cargo libgtk-4-dev
```

```bash
cargo build --release --locked
```

Das Ergebnis liegt unter `target/release/querencia-bilder`. Übersetzungen
(ohne Abhängigkeiten; GNU gettext ≥ 0.23 zum Gegenprüfen):

```bash
python3 werkzeug/i18n.py pruefen    # .pot aktuell, alle .po vollständig
python3 werkzeug/i18n.py mo <ziel>  # <ziel>/de/LC_MESSAGES/querencia-bilder.mo
python3 werkzeug/i18n.py desktop data/org.querencia.Bilder.desktop.in data/org.querencia.Bilder.desktop
```

Ein Paket legt die `.mo` nach `/usr/share/locale/<lang>/LC_MESSAGES/`; ohne
Katalog zeigt Bilder Englisch. Die Ressource mit dem Symbol des Info-Fensters
wird nach einer Änderung am SVG neu erzeugt (Befehl in
`data/resource/bilder.gresource.xml`); der Unit-Test `resource::tests`
schlägt fehl, solange sie veraltet ist. Symbole bewusst **ohne** 48×48
(GTK 4.22 stürzt ab, wenn ein Symbol in 48×48 und als Scalable vorliegt).

## Prüfen

```bash
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
python3 werkzeug/i18n.py pruefen
```

Diese Befehle laufen ohne Bildschirm; die ersten drei sind auch die CI
(`.github/workflows/ci.yml`). Die **GUI-Prüfung** läuft in einer
abgeschotteten Sitzung (eigener labwc ohne Bildschirm, eigenes `HOME` und
`XDG_RUNTIME_DIR`, virtuelle Eingabe), niemals in der laufenden
Desktopsitzung; ihr Werkzeug liegt noch außerhalb dieses Repos.
Arbeitsweise: [CONTRIBUTING.md](CONTRIBUTING.md).

## Vorbild und Abgrenzung

Vorbild für Bedienung und Aussehen ist **Eye of GNOME 2.30 aus GNOME 2.30**.
Querencia übernimmt davon die Bedienideen (Menüs, Kürzel, Anordnung) und den
englischen Wortschatz der Menüs, **keinen Quellcode, keine Grafiken**; aus
Eye of MATE, Eye of GNOME oder Ristretto wurde kein Code übernommen. Das
Projekt ist mit Eye of GNOME, GNOME und deren Autoren weder verbunden noch von
ihnen unterstützt.

Abweichungen vom Vorbild und Dinge, die bewusst fehlen:

- GTK 4 statt GTK 2, Fensterrahmen vom Compositor (labwc), kein libadwaita.
- Keine Bearbeitung und kein Speichern; Drehen wirkt nur auf die Anzeige.

Fremdbestandteile: nur Rust-Abhängigkeiten (gtk4-rs, gdk-pixbuf, gettext-rs
und Umfeld) unter MIT, MIT OR Apache-2.0 und verwandten freien Lizenzen; Liste
mit `cargo metadata --locked`. Dynamisch gelinkt: GTK 4, gdk-pixbuf und das
gettext der glibc.

## Lizenz

Programm: GPL-3.0-or-later, siehe [LICENSE](LICENSE). Copyright 2026
Querencia Desktop contributors; Urheberhinweis in
[COPYRIGHT](COPYRIGHT).

Name und Logo „Querencia“ sind keine freien Inhalte: [MARKE.md](MARKE.md).

## Fehler melden

Fehler, Wünsche und Fragen zu Querencia gehen an die gemeinsame Meldestelle:
<https://github.com/querencia-desktop/querencia/issues/new/choose>.
Sicherheitslücken bitte nicht öffentlich melden, siehe [SECURITY.md](SECURITY.md).
