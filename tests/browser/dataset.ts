/// The sample and the scene layout, worked out a second time.
///
/// Nothing here reads the application. That is the point: a sorted order or a
/// selected set that is compared against what the application itself produced
/// is compared against nothing. These are the same rules written again, in
/// another language, from the definition in the plan - so when the two agree
/// the agreement means something, and when they do not, one of them is wrong
/// and the difference says which rows.
///
/// The definitions are frozen for this phase; `loads.ts` carries the baseline
/// identifier that changes when they do.

export const ROWS = 100_000;
export const COLUMNS = 20;
export const CELL = 16;
export const ROW_BYTES = COLUMNS * CELL;
export const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
export const SEED = 0x9e37_79b9;

let sample: Uint8Array | null = null;

/// Thirty-two megabytes of cells, rows then columns then characters.
/// Generated once per test process: it takes about a second, and every check
/// in the phase compares against the same bytes.
export function cells(): Uint8Array {
    if (sample !== null) {
        return sample;
    }
    // xorshift32, inline: the sequence is the definition, and a generator
    // called thirty-two million times would cost more than the arithmetic.
    const bytes = new Uint8Array(ROWS * ROW_BYTES);
    const alphabet = new Uint8Array(ALPHABET.length);
    for (let i = 0; i < ALPHABET.length; i++) {
        alphabet[i] = ALPHABET.charCodeAt(i);
    }
    let x = SEED >>> 0;
    for (let i = 0; i < bytes.length; i++) {
        x ^= x << 13;
        x >>>= 0;
        x ^= x >>> 17;
        x ^= x << 5;
        x >>>= 0;
        bytes[i] = alphabet[x % 36];
    }
    sample = bytes;
    return bytes;
}

/// One cell, as the sixteen characters it is.
export function cell(row: number, column: number): string {
    const start = row * ROW_BYTES + column * CELL;
    return String.fromCharCode(...cells().subarray(start, start + CELL));
}

/// The identity of a row of the freshly generated sample. Identities start at
/// one and are handed out in order; nothing has been inserted or deleted yet.
export function rowId(row: number): number {
    return row + 1;
}

/// FNV-1a over every cell byte, as the decimal string the application reports.
///
/// Sixty-four bits in two halves rather than in `BigInt`: the loop runs
/// thirty-two million times, and a BigInt multiply each time round would make
/// this the slowest thing in the suite by two orders of magnitude. The halves
/// are joined once, at the end, where one BigInt costs nothing.
export function hash(bytes: Uint8Array = cells()): string {
    // 0xcbf29ce484222325, and the prime 0x100000001b3 as its two halves.
    let hi = 0xcbf2_9ce4;
    let lo = 0x8422_2325;
    for (let i = 0; i < bytes.length; i++) {
        lo = (lo ^ bytes[i]) >>> 0;
        const low = lo * 0x1b3;
        const carry = Math.floor(low / 4_294_967_296);
        const high = hi * 0x1b3 + lo * 0x100 + carry;
        lo = low >>> 0;
        hi = high >>> 0;
    }
    return ((BigInt(hi) << 32n) | BigInt(lo)).toString();
}

/// The row order a stable ascending sort on `column` produces, as row indices.
///
/// The keys are compared as strings because the alphabet is ASCII, so code
/// unit order and byte order are the same order. Equal keys keep the order
/// they were in, which is what the tie-break on the index does.
const orders = new Map<string, number[]>();

export function sorted(column: number, ascending = true): number[] {
    const key = `${column}:${ascending}`;
    const known = orders.get(key);
    if (known !== undefined) {
        return known;
    }
    const data = cells();
    const keys = new Array<string>(ROWS);
    for (let row = 0; row < ROWS; row++) {
        const start = row * ROW_BYTES + column * CELL;
        keys[row] = String.fromCharCode(...data.subarray(start, start + CELL));
    }
    const order = new Array<number>(ROWS);
    for (let row = 0; row < ROWS; row++) {
        order[row] = row;
    }
    const direction = ascending ? 1 : -1;
    order.sort((a, b) => {
        if (keys[a] < keys[b]) {
            return -direction;
        }
        if (keys[a] > keys[b]) {
            return direction;
        }
        return a - b;
    });
    orders.set(key, order);
    return order;
}

/// The scene, worked out the same way.
export const SCENE = {
    count: 10_000,
    columns: 100,
    width: 120,
    height: 24,
    stepX: 130,
    stepY: 40,
    overlayEvery: 10,
    overlayX: 60,
    overlayY: 12,
} as const;

export const EXTENT_X = (SCENE.columns - 1) * SCENE.stepX + SCENE.overlayX + SCENE.width;
export const EXTENT_Y =
    (SCENE.count / SCENE.columns) * SCENE.stepY - SCENE.stepY + SCENE.overlayY + SCENE.height;

export function isOverlay(index: number): boolean {
    return index % SCENE.overlayEvery === SCENE.overlayEvery - 1;
}

export function sceneRect(index: number): { x: number; y: number } {
    const column = index % SCENE.columns;
    const row = Math.floor(index / SCENE.columns);
    const shift = isOverlay(index) ? [SCENE.overlayX, SCENE.overlayY] : [0, 0];
    return { x: column * SCENE.stepX + shift[0], y: row * SCENE.stepY + shift[1] };
}

export function sceneLabel(index: number): string {
    return `OBJ${String(index).padStart(5, "0")}`;
}

/// How many objects a camera at `(x, y)` can see in a pane of this size. The
/// window is widened by one either way, so an object arrives at the edge
/// rather than appearing inside it.
export function sceneVisible(x: number, y: number, width: number, height: number): number {
    const rows = SCENE.count / SCENE.columns;
    const clampTo = (value: number, low: number, high: number) =>
        Math.min(Math.max(value, low), high);
    const firstColumn = clampTo(Math.floor(x / SCENE.stepX) - 1, 0, SCENE.columns - 1);
    const lastColumn = clampTo(
        Math.ceil((x + width) / SCENE.stepX),
        firstColumn,
        SCENE.columns - 1
    );
    const firstRow = clampTo(Math.floor(y / SCENE.stepY) - 1, 0, rows - 1);
    const lastRow = clampTo(Math.ceil((y + height) / SCENE.stepY), firstRow, rows - 1);
    return (lastColumn + 1 - firstColumn) * (lastRow + 1 - firstRow);
}

export function sceneClamp(
    x: number,
    y: number,
    width: number,
    height: number
): [number, number] {
    const to = (value: number, extent: number, pane: number) =>
        Math.min(Math.max(value, 0), Math.max(extent - pane, 0));
    return [to(x, EXTENT_X, width), to(y, EXTENT_Y, height)];
}
