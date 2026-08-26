# Styled bidirectional text flow

## Mapping

`CAP-TXT-001`: The system must render styled bidirectional text from fonts registered at runtime.

## Behavior

```mermaid
flowchart LR
    Font[Runtime font registration] -->|validated asset| Text[Text and editing]
    Spans[Styled bidirectional spans] -->|layout request| Text
    Locale[Locale and direction] -->|context event| Text
    Text -->|shaped lines and runs| Scene[Scene composition]
    Text -->|owned font lifetime| Resources[Asset and resource manager]
```

## Failure path

If a font is malformed, released too early, or lacks a required fallback, Text and editing returns a structured text-layout error and doesn't publish partial runs.
