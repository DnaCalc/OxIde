# oxide-host-bridge

Status: `w343_host_bridge_scaffold`

Host-neutral service facade for shared OxIde UI consumers.

Consumers:

- DnaOxIde / DNA OxIde Tauri host adapters,
- standalone browser/WASM review fixtures,
- future DnaOneCalc host mounts,
- `oxide-guilab` scenario fixtures.

Boundary rules:

- no Tauri dependency,
- no app-folder dependency,
- no sibling repo writes,
- no CLI/LSP routing for internal OxIde semantics,
- no duplicated final OxVba DTO truth,
- unavailable or subset-backed services must keep no-claim flags explicit.

See [`../../docs/HOST_BRIDGE_SERVICE_MAP.md`](../../docs/HOST_BRIDGE_SERVICE_MAP.md).
