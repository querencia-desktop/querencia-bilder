# Prüfen – Querencia Bilder

```bash
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked                       # Unit-Tests (Ordnerliste, Zoom, Größen, Ressource, Katalog)
python3 werkzeug/i18n.py pruefen          # Katalog vollständig, muss PASS melden
```

GUI-Prüfung nur isoliert (headless labwc, eigener D-Bus, virtuelle Eingabe),
nie in der laufenden Sitzung. Die Prüfgerüste liegen noch außerhalb dieses
Repos:

| Fassung | Prüfgerüst | Bericht |
|---|---|---|
| QI-01 (Prototyp) | Arbeitsordner des Prototyps (nicht im Repo) |  |
| 0.1.1 | Arbeitsordner der Fassung, `werkzeug/` (nicht im Repo) |  |
| 0.2.0 (je Sprache) | Arbeitsordner der Fassung, `werkzeug/` (nicht im Repo) |  |
| 0.2.1 (CSS-Welle) | Arbeitsordner der Fassung (nicht im Repo) |  |
| 0.2.2 (Menü Grafik) | Arbeitsordner der Fassung, `werkzeug/` (nicht im Repo) |  |
| 0.2.4 (dunkel) | Arbeitsordner der Fassung, `werkzeug/` (nicht im Repo) |  |
| 0.2.5 (Leerhinweis) | Arbeitsordner der Fassung, `werkzeug/` (nicht im Repo) |  |
| 0.2.7 (it/es/fr) | Prüfgerüst von 0.2.6 erweitert um it/es/fr und ein Szenario für alle Menüs (nicht im Repo) |  |
