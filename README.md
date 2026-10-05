# Saytrue

> **Die rechten Worte finden**

[![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)](#stand)
[![Release](https://img.shields.io/badge/release-in%20Vorbereitung-yellow.svg)](#stand)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)](#plattformen)
[![Languages](https://img.shields.io/badge/i18n-15%20languages-blue.svg)](#weitere-funktionen)

Saytrue ist eine Desktop-App, die vorschlägt, wie du etwas anders sagen kannst: Du sprichst oder tippst einen Text, und die App formuliert ihn um, überschrieben mit dem, was die neue Fassung anders macht, etwa „Wärmer formuliert“ oder, in Pro, „Beobachtung statt Bewertung“ nach der Gewaltfreien Kommunikation.
Saytrue ist noch nicht veröffentlicht: Es gibt weder einen Store-Eintrag noch einen Download.

## Inhalt

- [Stand](#stand)
- [So funktioniert es](#so-funktioniert-es)
- [Linsen und Umformulierungen](#linsen-und-umformulierungen)
- [Weitere Funktionen](#weitere-funktionen)
- [Free und Pro](#free-und-pro)
- [Plattformen](#plattformen)
- [Sprachmodelle](#sprachmodelle)
- [Datenschutz](#datenschutz)
- [Was Saytrue nicht tut](#was-saytrue-nicht-tut)
- [Lizenz](#lizenz)

---

## Stand

Version 0.0.1, nicht veröffentlicht. Die Build-Workflows für macOS, Windows und Linux sind eingerichtet, ein Release gab es noch nicht. Die Vertriebswege unter [Plattformen](#plattformen) sind geplant; Links und Installationsbefehle folgen mit der ersten Veröffentlichung. iOS und Android sind in Arbeit.

Preis und Geschäftsmodell stehen noch nicht fest.

---

## So funktioniert es

1. **Eingabe:** Aufnahme über das Mikrofon, eine Audiodatei oder getippter Text. Gesprochenes wird lokal transkribiert.
2. **Linsen wählen:** Eine Linse ist ein psychologischer Ansatz, unter dem Saytrue den Text betrachtet. Für jede Linse stellst du ein: **Aus**, **Analyse** oder **Vorschlag** (Vorschlag nur bei Linsen mit Umformulierung, siehe Tabelle).
3. **Ergebnis im Verlauf:** Bei **Vorschlag** erscheint eine Umformulierung. Über ihr steht, was sie anders macht („Beobachtung statt Bewertung“), darunter ein Satz Begründung. Den Namen des Modells musst du dafür nicht kennen. Bei **Analyse** siehst du das Ergebnis der Analyse selbst, zum Beispiel die vier Seiten einer Nachricht.

Beim Text- und Audiodatei-Import bestimmen die Linsen, welche Analysen laufen. Bei der Live-Aufnahme legen das die Analyse-Einstellungen fest; die Umformulierungen folgen auch dort den Linsen.

Ist ein Text zu kurz, analysiert Saytrue ihn nicht und nennt den Grund. In Pro prüft zusätzlich ein Wächter je Analyse, ob der Text genug dafür hergibt, und überspringt sie sonst mit Begründung.

---

## Linsen und Umformulierungen

| Linse | Grundlage | Umformulierung | Tier |
|-------|-----------|----------------|------|
| Emotion | Plutchik, Russell; Stimme und Text | keine | Free |
| Ton | Formalität, Nähe, Direktheit, Energie, Stimmung | „Wärmer formuliert“ | Free |
| Thema | sieben Kategorien | keine | Free |
| Fehlschluss | 16 Typen von Argumentationsfehlern | „Mit konkretem Beispiel“ | Pro |
| Vier Seiten | Schulz von Thun | „Mit klarem Appell“ | Pro |
| Gewaltfreie Kommunikation | Rosenberg | „Beobachtung statt Bewertung“ | Pro |
| Kognitive Verzerrungen | Beck | „Ohne Übergeneralisierung“ | Pro |
| Transaktionsanalyse | Berne | „Auf Augenhöhe“ | Pro |
| Bewertung (Appraisal) | Lazarus, Scherer | „Aktiv gestaltbar formuliert“ | Pro |
| Regulatorischer Fokus | Higgins | „Auf Chancen ausgerichtet“ | Pro |
| Sprechertrennung | nur im Modus Gespräch | keine | Pro |

---

## Weitere Funktionen

- **Formulierungsalternativen (Pro):** drei Fassungen derselben Aussage, direkt, empathisch und deeskalierend. Im Verlauf per Knopf, für Text aus anderen Programmen über das macOS-Dienste-Menü oder ein Tastenkürzel, das den markierten Text in Saytrue oder die Zwischenablage nimmt. In den Store-Ausgaben wirkt das Tastenkürzel nur, solange Saytrue im Vordergrund ist.
- **Coaching (Pro):** fasst auf Anforderung die Ergebnisse der Analysen zu Hinweisen für das eigene Sprechen zusammen.
- **Reflexionsmodus (Pro):** Selbstreflexion (eigener Text), Fremdreflexion (zum Beispiel eine erhaltene Nachricht) und Gespräch (mehrere Sprecher).
- **Bibelimpuls und Gebet (Free, standardmäßig aus):** ein bis drei thematisch passende Bibelstellen (Schlachter 2000, lokal gespeichert) und ein kurzes Gebet zum Text.
- **Diktat und Schnellaufnahme:** Nur-Transkription mit gesprochener Zeichensetzung; Schnellaufnahme über das Tray-Menü legt den Text in die Zwischenablage (nicht unter Windows).
- **Hilfe-Chat:** beantwortet Fragen zur App und zu den Ansätzen aus einer lokalen Wissensbasis.
- **Speichern und Export:** Aufnahmen werden lokal gespeichert; der Verlauf lässt sich als Markdown, Text, PDF, HTML oder Word exportieren.
- **15 Sprachen:** Deutsch, Englisch, Spanisch, Französisch, Italienisch, Niederländisch, Portugiesisch, Polnisch, Schwedisch, Dänisch, Norwegisch, Tschechisch, Rumänisch, Russisch und Japanisch, in Oberfläche, Prompts und Transkription.

---

## Free und Pro

Free und Pro sind getrennte Ausgaben der App. Unter Linux gibt es nur Pro, weil dort kein Store einen späteren Kauf ermöglicht.

| | Free | Pro |
|---|---|---|
| Transkription, Diktat | ✅ | ✅ |
| Linsen Emotion, Ton, Thema | ✅ | ✅ |
| Umformulierung | eine („Wärmer formuliert“) | alle acht Linsen mit Umformulierung |
| Weitere Linsen (siehe Tabelle oben) | ❌ | ✅ |
| Formulierungsalternativen in drei Stilen | ❌ | ✅ |
| Coaching | ❌ | ✅ |
| Fremdreflexion und Gespräch | ❌ | ✅ |
| Wächter je Analyse | ❌ (nur Mindestlänge) | ✅ |
| Bibelimpuls und Gebet | ✅ | ✅ |
| Hilfe-Chat, Export | ✅ | ✅ |

---

## Plattformen

| Plattform | Architektur | Geplanter Vertrieb | Ausgabe |
|-----------|-------------|--------------------|---------|
| macOS | Apple Silicon, Intel | Mac App Store | Free, Pro |
| macOS | Apple Silicon, Intel | Homebrew | Free |
| Windows | x64 | Microsoft Store | Free, Pro |
| Windows | x64 | Installer (NSIS), winget | Free |
| Linux | x64, ARM64 | .deb, .rpm, .AppImage | Pro |
| Linux | x64 | Snap | Pro |

Stand aller Wege: geplant, noch nichts veröffentlicht. macOS mit Apple Silicon ist die Hauptplattform der Entwicklung. Unterschiede je Plattform: MLX-Whisper nur auf Apple Silicon; der App-Store-Build hat statt des globalen Tastenkürzels eines innerhalb der App; Store-Ausgaben aktualisieren sich über den Store.

---

## Sprachmodelle

**Transkription:** lokal mit whisper.cpp (german-turbo für Deutsch, kotoba-v2 für Japanisch, large-v3-turbo für die übrigen Sprachen), auf Apple Silicon wahlweise MLX-Whisper. Optional in der Cloud über OpenAI, nur mit ausdrücklicher Einwilligung, weil die Stimme ein biometrisches Datum ist.

**Analyse und Umformulierung:**

| Anbieter | Ort | Hinweis |
|----------|-----|---------|
| Ollama | lokal | Standard; empfohlenes Modell `qwen3:4b-custom`, außerdem 1.5B, 7B und `qwen3:8b` |
| Apple Intelligence | lokal | ab macOS 26 auf Apple Silicon |
| OpenAI | Cloud (USA) | nur mit Einwilligung |
| Anthropic | Cloud (USA) | nur mit Einwilligung |
| Mistral | Cloud (EU, Paris) | nur mit Einwilligung |

Ohne Sprachmodell funktioniert nur die Transkription. Die App führt beim ersten Start durch die Einrichtung.

---

## Datenschutz

- **Vollständig lokal möglich:** whisper.cpp und Ollama oder Apple Intelligence arbeiten nach dem Modell-Download ohne Netz.
- **Cloud nur mit Einwilligung:** Vor der ersten Nutzung eines Cloud-Anbieters holt die App eine Einwilligung ein, die nennt, welche Inhalte übertragen werden; für Cloud-Transkription gibt es eine eigene Einwilligung. Wer zu lokaler Verarbeitung wechselt, widerruft damit die Einwilligung.
- **Lokale Speicherung:** Aufnahmen und Ergebnisse liegen auf dem Gerät (macOS `~/Library/Application Support/Saytrue/recordings/`, Windows `%LOCALAPPDATA%\Saytrue\recordings\`, Linux `~/.local/share/saytrue/recordings/`). Es gibt keine Cloud-Datenbank.
- **API-Schlüssel** liegen im Schlüsselbund des Betriebssystems (Keychain, Credential Manager, Secret Service).
- **Fehlerberichte** sind optional und standardmäßig aus.

Datenschutzerklärung: [saytrue.de/datenschutz](https://saytrue.de/datenschutz/)

---

## Was Saytrue nicht tut

Saytrue beschreibt Sprache und schlägt Formulierungen vor. Es stellt keine Diagnosen, gibt keine klinischen Werte aus, bietet keine Therapie und keine Krisenhilfe. Ein Filter prüft die Ausgaben der Sprachmodelle und blockiert solche Inhalte. KI-Ergebnisse können fehlerhaft sein.

In einer Krise: Telefonseelsorge 0800 111 0 111 (Deutschland, rund um die Uhr, kostenlos).

---

## Lizenz

Alle Rechte vorbehalten. Nutzungsbedingungen: [saytrue.de/eula](https://saytrue.de/eula/)

---

**Autor:** Marc Allgeier | **Version:** 0.0.1
