/// What the PRD's fixed loads are, in one place.
///
/// A budget quoted without the load it was measured on is a number with no
/// meaning: B2's frame interval is about a hundred thousand rows scrolled
/// three rows a frame, and the same figure on ten rows would be a different
/// claim entirely. So the loads are written down here, and every report says
/// which baseline it was measured against.
///
/// The definitions are frozen for this phase. Changing a seed, a layout
/// constant, a count or an action list changes what the measurements mean, so
/// it changes `BASELINE` too and the earlier figures stop being comparable.

import { EXTENT_X, EXTENT_Y, ROWS, COLUMNS, SEED, SCENE } from "./dataset";

/// The identifier every report prints beside its figures. It names the loads,
/// not the build: a new build measured against the same loads is comparable,
/// and a new load is not.
export const BASELINE = "p3-loads-1";

/// B0: the smallest complete application. Ten DOM controls, twenty GPU
/// controls in one region, one shared state, Latin text only.
export const B0 = {
    example: "fusion-basic",
    domControls: 10,
    gpuControls: 20,
    regions: 1,
    scopes: 1,
} as const;

/// B2: the large table.
export const B2 = {
    example: "data-workbench",
    rows: ROWS,
    columns: COLUMNS,
    seed: SEED,
    /// Rows the scroll driver advances by on each frame it drives.
    scrollRowsPerFrame: 3,
    /// The three rows every keyboard and query journey has to reach.
    targets: [1, 50_000, ROWS],
} as const;

/// B3: the large scene.
export const B3 = {
    example: "data-workbench",
    objects: SCENE.count,
    labelLength: 8,
    layers: 2,
    extent: [EXTENT_X, EXTENT_Y],
    /// Scene pixels the pan driver moves the camera by on each driven frame.
    panPerFrame: [7, 3],
    /// The three objects every query journey has to reach.
    targets: ["OBJ00000", "OBJ05000", "OBJ09999"],
} as const;

/// B4: two instances, two regions each, mounted and unmounted in rounds.
///
/// The instances are booted once and live for the whole measurement: a round
/// mounts and unmounts scopes inside them. Restarting an instance is a
/// different measurement, because a dead instance's memory never comes back.
export const B4 = {
    example: "fusion-basic",
    instances: 2,
    regionsPerInstance: 2,
    warmupRounds: 20,
    measuredRounds: 80,
    /// The endurance run: two hours at ten actions a second.
    actionsPerSecond: 10,
    minutes: 120,
} as const;

/// The rotation the endurance run drives, by instance and region. Every action
/// is counted out and counted back in; the two totals have to match.
export const B4_ACTIONS = [
    { instance: 1, region: 1, action: "dom-increment" },
    { instance: 1, region: 2, action: "gpu-increment" },
    { instance: 2, region: 1, action: "dom-increment" },
    { instance: 2, region: 2, action: "gpu-increment" },
] as const;

/// The twenty things a person has to be able to find by role and name.
///
/// Twenty is the PRD's number (R25), and the list is frozen here so that the
/// check is against a list somebody wrote down rather than against whatever
/// the application happens to expose. `from` says which milestone builds the
/// element: until then the entry is a commitment, not a passing check. The
/// table's own controls arrive with the table (M2); the ones that drive a job
/// arrive with the jobs (M3); the scene's arrive with the scene (M4).
export interface Locator {
    testId: string;
    role: string;
    name: string;
    side: "table" | "scene";
    from: "M2" | "M3" | "M4";
}

export const LOCATORS: Locator[] = [
    { testId: "table", role: "grid", name: "the sample", side: "table", from: "M2" },
    { testId: "table-goto", role: "spinbutton", name: "go to row", side: "table", from: "M2" },
    { testId: "table-insert", role: "button", name: "insert 10", side: "table", from: "M2" },
    { testId: "table-delete", role: "button", name: "delete 10", side: "table", from: "M2" },
    { testId: "table-status", role: "status", name: "", side: "table", from: "M2" },
    { testId: "table-column-0", role: "columnheader", name: "column 1", side: "table", from: "M2" },
    { testId: "table-column-1", role: "columnheader", name: "column 2", side: "table", from: "M2" },
    { testId: "table-tree", role: "tree", name: "groups", side: "table", from: "M2" },
    { testId: "table-tree-g0", role: "treeitem", name: "group 1", side: "table", from: "M2" },
    { testId: "table-tree-g0-0", role: "treeitem", name: "group 1.1", side: "table", from: "M2" },
    { testId: "table-detail-0", role: "textbox", name: "column 1", side: "table", from: "M2" },
    { testId: "table-detail-submit", role: "button", name: "save", side: "table", from: "M2" },
    { testId: "table-find", role: "searchbox", name: "find", side: "table", from: "M3" },
    { testId: "table-filter", role: "searchbox", name: "filter", side: "table", from: "M3" },
    { testId: "scene-find", role: "searchbox", name: "find an object", side: "scene", from: "M4" },
    { testId: "scene-selected", role: "list", name: "selected", side: "scene", from: "M4" },
    { testId: "scene-selected-count", role: "status", name: "", side: "scene", from: "M4" },
    { testId: "scene-detail-label", role: "textbox", name: "label", side: "scene", from: "M4" },
    { testId: "scene-detail-submit", role: "button", name: "save", side: "scene", from: "M4" },
    { testId: "scene-clear", role: "button", name: "clear the selection", side: "scene", from: "M4" },
];
