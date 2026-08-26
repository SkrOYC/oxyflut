# Domain model

The model describes product concepts and their relationships. It doesn't define implementation modules or processes.

```mermaid
erDiagram
    APPLICATION_DEVELOPER ||--o{ APPLICATION : builds
    APPLICATION ||--|| APPLICATION_RUNTIME : uses
    APPLICATION_RUNTIME ||--o{ COMPONENT : manages
    COMPONENT ||--o{ COMPONENT : composes
    COMPONENT ||--o{ VIEW : contributes-to
    VIEW }o--o| WINDOW : associated-with
    WINDOW }o--|| OPERATING_ENVIRONMENT : participates-in
    VIEW ||--o{ DISPLAY_EPOCH : has
    DISPLAY_EPOCH ||--o{ PRESENTATION_OPPORTUNITY : contains
    VIEW ||--|| SEMANTICS_TREE : owns
    COMPONENT }o--|| SEMANTICS_TREE : contributes-to
    INPUT_METHOD_EDITOR }o--|| VIEW : edits
    APPLICATION_RUNTIME }o--|| RENDERING_SUBSTRATE : renders-through
    SUBSTRATE_CANDIDATE ||--|| RENDERING_SUBSTRATE : proposes
    CAPABILITY_BASELINE }o--o{ SUBSTRATE_CANDIDATE : evaluates
    TEST_AND_VERIFICATION_HARNESS }o--o{ CAPABILITY_BASELINE : verifies
    TEST_AND_VERIFICATION_HARNESS }o--o{ DISPLAY_EPOCH : observes
    RELEASE_MAINTAINER ||--o{ SUBSTRATE_CANDIDATE : qualifies
    SURFACE_RECOVERY }o--|| VIEW : restores
    LOCAL_DIAGNOSTICS }o--|| APPLICATION_RUNTIME : describes
```

## Relationship rules

- A view owns independent focus, metrics, semantics identity, lifecycle, and presentation state.
- A component can contribute visual and semantic content to a view without owning the operating environment.
- A capability baseline evaluates complete substrate candidates. It doesn't waive product capabilities for one candidate.
- A display epoch ends when display association or timing mode changes.
- Surface recovery preserves application-runtime state unless the operating environment explicitly invalidates that state.
- Local diagnostics remain inside the declared machine-local trust boundary.
- Exported telemetry crosses that boundary and remains outside P0.
