# Handoff — W340 Cross-Repo Native Service Or Host Implementation

Status: `blocked_pending_explicit_authorization`
Date: 2026-05-07
Source workset: W330 — OxVba Native Runtime Service Contract

## Decision

The next meaningful capability step is cross-repo:

1. implement or expose an OxVba native runtime/debug/Immediate service that can satisfy W330 packets, or
2. implement the paired DnaOneCalc web-shell host mount described by W310.

Both options require sibling-repository writes. Current OxIde agent guardrails allow reading sibling repositories but forbid writing to them unless explicitly authorized by the user.

## Current OxIde State

OxIde now has:

- disk-backed native filesystem/session persistence proof in W320,
- typed runtime service contract packets in W330,
- GUI-lab scenarios for browser-disabled/native-service-missing runtime, Immediate, and debug states,
- no fake runtime/debug/Immediate data,
- no COM runtime claim.

Relevant OxIde evidence commands:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-runtime-service-contract-native-missing
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-service-contract-native-missing
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-service-contract-native-missing
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-host-contract
```

## Option A — OxVba Native Runtime Service

Goal:

```text
OxVba exposes a native service/API that can provide real runtime, Immediate, and debug packets for ThinSliceHello.
```

Minimum required from OxVba or a shared DNA Calc crate:

- runtime session id,
- workspace/project/run target correlation,
- run command availability and disabled reasons,
- runtime lifecycle/activity/output/error events,
- Immediate request/response events,
- debug session state,
- debug command availability,
- callstack/locals/watches/breakpoint DTOs,
- native service missing/error taxonomy.

OxIde acceptance after that should flip only the claims proven by tests. COM runtime must remain false unless a tested COM service exists.

## Option B — DnaOneCalc Web-Shell Host Mount

Goal:

```text
DnaOneCalc mounts OxIde's W310/W300 web-shell contract in its real product shell.
```

Minimum required from DnaOneCalc:

- consume `DnaOneCalcWebShellHostPacket` or serialized equivalent,
- mount `GuiShellPacket`/`oxide-webshell` output,
- prove host DOM shows ThinSliceHello and Module1.bas,
- preserve OxIde/OxVba/DnaOneCalc ownership boundaries,
- keep filesystem/native runtime/COM/DOM-audit claims false unless independently tested.

Use `docs/HANDOFF_DNAONECALC_WEB_SHELL_HOST_API.md` as the DnaOneCalc-side API handoff.

## Authorization Needed

Before W340 can proceed beyond planning, the user must explicitly authorize sibling repo writes and name the target repo(s), for example:

```text
Authorize writes to C:/Work/DnaCalc/OxVba for W340 OxVba native runtime service work.
```

or

```text
Authorize writes to C:/Work/DnaCalc/DnaOneCalc for W340 DnaOneCalc host mount work.
```

Without that authorization, further OxIde-only work in this lane risks drifting into fake native-service behavior.

## Not Claimed Yet

- real OxVba runtime execution,
- real Immediate responses,
- real debug data,
- native COM discovery/invocation,
- real DnaOneCalc browser host mount,
- full DOM accessibility audit/compliance.
