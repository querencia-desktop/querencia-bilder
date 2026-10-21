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
| QI-01 (Prototyp) | `~/projects/querencia-werkstatt-w5-bilder/werkzeug/` |  |
| 0.1.1 | `~/projects/querencia-bilder-011/werkzeug/` |  |
| 0.2.0 (je Sprache) | `~/projects/querencia-bilder-020/werkzeug/` |  |
| 0.2.1 (CSS-Welle) | `~/projects/querencia-bilder-css/` |  |
| 0.2.2 (Menü Grafik) | `~/projects/querencia-bilder-022/werkzeug/` |  |
| 0.2.4 (dunkel) | `~/projects/querencia-bilder-024/werkzeug/` |  |
| 0.2.5 (Leerhinweis) | `~/projects/querencia-bilder-025/werkzeug/` |  |
