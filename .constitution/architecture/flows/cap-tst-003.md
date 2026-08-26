# Fault-injection flow

## Mapping

`CAP-TST-003`: The test harness must inject every supported recoverable lifecycle and graphics fault.

## Behavior

```mermaid
flowchart LR
    Case[Frozen supported fault case] -->|fault-injection command| Environment[Operating environment or substrate mechanism]
    Environment -->|externally observed fault event| Recovery[View coordinator recovery]
    Recovery -->|state and resource evidence| Harness[Test and verification harness]
    Baseline[Platform capability baseline] -->|applicability input| Harness
    Harness -->|deadline and invariant result| Gate[Recovery gate]
```

## Failure path

If the fault cannot be induced despite platform support, evidence is incomplete and remains gating. Unsupported events require cited baseline evidence.
