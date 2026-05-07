import {
  DNA_OXIDE_INTERACTION_HARNESS,
  FOCUS_ROUTE,
  createInteractionHarness,
  renderedMarkupContainsRoles,
  walkFocusRoute
} from "../src/interaction-harness.js";

const state = createInteractionHarness();
const route = walkFocusRoute(state);

if (route.length !== FOCUS_ROUTE.length) {
  console.error(`Focus route length mismatch: ${route.length} vs ${FOCUS_ROUTE.length}`);
  process.exit(1);
}

for (let index = 0; index < FOCUS_ROUTE.length; index += 1) {
  if (route[index] !== FOCUS_ROUTE[index]) {
    console.error(`Focus route mismatch at ${index}: ${route[index]} vs ${FOCUS_ROUTE[index]}`);
    process.exit(1);
  }
}

if (!renderedMarkupContainsRoles(state.markup, FOCUS_ROUTE)) {
  console.error("W345 host shell markup is missing one or more focus route roles");
  process.exit(1);
}

if (DNA_OXIDE_INTERACTION_HARNESS.fullDomAccessibilityAuditClaimed !== false
  || DNA_OXIDE_INTERACTION_HARNESS.browserEventLoopDriven !== false
  || DNA_OXIDE_INTERACTION_HARNESS.playwrightOrWebDriverDriven !== false) {
  console.error("Focus harness overclaimed browser/accessibility coverage");
  process.exit(1);
}

const requiredRoles = [
  "host-project-spine",
  "host-editor-boundary",
  "host-diagnostics-panel",
  "host-lifecycle-panel",
  "host-command-palette",
  "host-runtime-panel",
  "host-immediate-panel",
  "host-debug-panel",
  "host-com-panel",
  "host-claim-boundaries"
];

for (const role of requiredRoles) {
  if (!state.markup.includes(`role=\"${role}\"`)) {
    console.error(`Rendered host shell missing role ${role}`);
    process.exit(1);
  }
}

console.log(`DNA OxIde focus/no-mouse interaction verification passed: ${route.join(" -> ")}`);
