# Handoff — DnaOneCalc Web Shell Host API

Status: `handoff_ready`
Date: 2026-05-07
Source workset: W310 — DnaOneCalc Web Shell Hosting

## Summary

OxIde now has an OxIde-side DnaOneCalc web-shell host contract:

- `oxide-bridge::DnaOneCalcWebShellHostPacket`,
- `gui-dnaonecalc-web-shell-host-contract`,
- `gui-dnaonecalc-web-shell-dom-readiness`.

This handoff describes the DnaOneCalc-side API and smoke-test expectations needed for a paired host implementation. No DnaOneCalc repository files were modified by W310.

## Required Inputs

A DnaOneCalc host mount should receive these OxIde-owned contracts:

```text
DnaOneCalcWebShellHostPacket
  embedded_ide: EmbeddedIdePacket
  web_shell.state_contract: GuiShellPacket
  web_shell.adapter_crate: oxide-webshell
  web_shell.dom_readiness: parsed-html readiness summary
```

Minimum mount inputs visible in the W310 lab:

```text
EmbeddedIdePacket
GuiShellPacket
oxide-webshell snapshot or mounted component
```

DnaOneCalc should treat these as OxIde-owned inputs. DnaOneCalc owns host placement, product shell, and host policy, not OxIde IDE state.

## Candidate Host API Shape

A DnaOneCalc-side mount can be shaped as either a component prop or host service call. Names are suggestions, not imposed public API:

```rust
pub struct DnaOneCalcOxIdeWebShellProps {
    pub host_contract: DnaOneCalcWebShellHostPacket,
    pub shell_packet: GuiShellPacket,
}

pub fn mount_oxide_web_shell(props: DnaOneCalcOxIdeWebShellProps) -> HostMountResult;
```

If DnaOneCalc cannot depend directly on OxIde crates, use serialized JSON with the same packet boundaries:

```text
host_contract_json: DnaOneCalcWebShellHostPacket
shell_packet_json: GuiShellPacket
```

## Required Host Smoke Tokens

A paired DnaOneCalc smoke should prove only what it actually mounts. Suggested tokens:

```text
data-host="DnaOneCalc"
data-state-contract="GuiShellPacket"
data-embedding-contract="EmbeddedIdePacket"
data-web-adapter="oxide-webshell"
data-sibling-repo-writes="false" (or true only in the paired DnaOneCalc commit evidence)
data-host-mount-claimed="true" only after the DnaOneCalc smoke mounts it
ThinSliceHello
Module1.bas
project-spine
source-editor
diagnostics
document-lifecycle
run-output
capability-footer
```

Required no-claim tokens until corresponding tests exist:

```text
data-filesystem-persistence="false"
data-native-runtime="false"
data-com-runtime="false"
data-dom-audited="false"
DnaOneCalc browser host smoke is not claimed
```

Once a real DnaOneCalc host smoke exists, replace only the host-smoke claim with evidence. Do not change filesystem/runtime/COM/accessibility-audit claims without separate tests.

## Ownership Boundaries To Preserve

- DnaOneCalc: product shell, host placement, host policy, and persistence policy.
- OxIde: IDE experience, `GuiShellPacket`, web-shell adapter contract, command/focus/accessibility projection.
- OxVba: VBA project truth, language service, semantics, runtime, Immediate, debug, and COM truth.

## Capability Limitations

W310 and W300 do not prove:

- real DnaOneCalc browser hosting,
- full DOM accessibility audit/compliance,
- filesystem persistence,
- native OxVba runtime/debug/Immediate,
- native COM discovery or invocation.

A paired DnaOneCalc implementation must keep these limitations visible unless it adds matching tests.

## Suggested Paired DnaOneCalc Tests

1. Mount receives `DnaOneCalcWebShellHostPacket` and `GuiShellPacket`.
2. Host DOM shows `ThinSliceHello` and `Module1.bas`.
3. Host DOM shows the OxIde-owned surface slots.
4. Host DOM exposes ownership boundaries for DnaOneCalc, OxIde, and OxVba.
5. Host DOM shows W300 parsed DOM readiness as OxIde-side readiness.
6. Host DOM keeps filesystem/native runtime/COM/DOM-audit claims false.

## OxIde Evidence Source

Relevant OxIde lab scenarios:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-host-contract
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-dom-readiness
```

Relevant OxIde tests:

```powershell
cargo test --manifest-path crates/Cargo.toml -p oxide-bridge
cargo test --manifest-path crates/Cargo.toml -p oxide-guilab
```
