# Bauen und Übersetzen – Querencia Bilder

```bash
cargo build --release --locked
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
