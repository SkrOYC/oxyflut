# Rich-text editing flow

## Mapping

`CAP-TXT-003`: The system must provide insertion, replacement, grapheme and word deletion, undo, redo, and keyboard and pointer selection for rich text.

## Behavior

```mermaid
flowchart LR
    Command[Insert, replace, delete, undo, redo, or selection command] -->|in-process call| Validate[Text-unit and owner validation]
    Validate -->|transaction| Model[Editable text model]
    Model -->|history update| Undo[Bounded undo state]
    Model -->|selection and text change| Layout[Text layout]
    Layout -->|invalidation event| View[View coordinator]
```

## Failure path

If a command has stale selection, invalid text boundaries, or an unavailable history entry, the model preserves the prior text and selection and returns a structured error.
