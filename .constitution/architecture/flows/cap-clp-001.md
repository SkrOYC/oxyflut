# Clipboard editing flow

## Mapping

`CAP-CLP-001`: The system must provide copy, cut, and paste while preserving rich-text selection behavior and private-content boundaries.

## Behavior

```mermaid
flowchart LR
    Command[Copy, cut, or paste command] -->|focused edit event| Branch{Operation}
    Branch -->|copy or cut| Selection[Owned rich-text selection]
    Selection -->|private service request| Platform[Platform integration]
    Branch -->|paste| Platform
    Platform -->|validated private result| Edit[Editable text transaction]
    Edit -->|cut or paste commit| History[Undo and selection state]
```

## Failure path

If focus, permission, content size, format, or selection generation is invalid, the operation leaves text and clipboard ownership unchanged and records no raw content.
