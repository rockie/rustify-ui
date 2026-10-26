import {test, expect, waitForReady} from "./support";
import type {Page} from "@playwright/test";
import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";

test.use({fresh:true});
const key = "rustify-ui.theme-studio.v2";
const legacyKey = "rustify-ui.theme-studio.v1";
const prefix = `${legacyKey}.backup.`;
const lock = "rustify-ui.theme-studio.storage";
const snapshot = (page:Page) => page.evaluate(() => window.__theme_studio.snapshot());
const raw = (page:Page, name = key) => page.evaluate(name => localStorage.getItem(name), name);
const backupKey = (value:string) => prefix + createHash("sha256").update(value).digest("hex");
const backups = (page:Page) => page.evaluate(prefix => Object.fromEntries(Object.keys(localStorage).filter(name => name.startsWith(prefix)).sort().map(name => [name, localStorage.getItem(name)])), prefix);
async function ready(page:Page) {
  await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {timeout:60_000});
}
async function idle(page:Page) {
  await expect.poll(async () => {
    const state = await snapshot(page);
    return state.storage.permission === "Ready" && !state.storage.paused && state.storage.pending === 0 && state.storage.acknowledged?.epoch === state.editor_epoch;
  }).toBe(true);
}
async function edit(page:Page, value:string) {
  await page.getByTestId("editor-colors").click();
  await page.getByTestId("color-search").fill("");
  await page.getByTestId("color-token-primary").click();
  await page.getByTestId("color-value").fill(value);
  await page.getByTestId("color-value").press("Enter");
  await expect.poll(async () => {const s=await snapshot(page);return s.values[s.mode].primary;}).toBe(value);
}
async function save(page:Page, name:string) {
  await page.getByTestId("theme-save").click();
  await page.getByTestId("theme-name").fill(name);
  await page.getByTestId("theme-name-confirm").click();
}
async function second(page:Page) {
  const other = await page.context().newPage();
  await other.goto(page.url());
  await ready(other);
  return other;
}
async function hold(other:Page) {
  await other.evaluate(name => {
    const owner = window as any;
    owner.__storageHeld = false;
    owner.__storageLockDone = navigator.locks.request(name, async () => {
      owner.__storageHeld = true;
      await new Promise<void>(resolve => { owner.__storageRelease = resolve; });
    });
  }, lock);
  await expect.poll(() => other.evaluate(() => (window as any).__storageHeld)).toBe(true);
}
async function release(other:Page) {
  await other.evaluate(async () => {
    (window as any).__storageRelease();
    await (window as any).__storageLockDone;
  });
}
async function download(page:Page, id:string) {
  const pending = page.waitForEvent("download");
  await page.getByTestId(id).click();
  const artifact = await pending;
  return readFile((await artifact.path())!, "utf8");
}
async function seedLegacy(page:Page) {
  await waitForReady(page);
  const before = await snapshot(page);
  const document = {schema_version:1, id:"legacy-draft", name:"旧草稿", styles:before.values};
  const saved = [
    {...structuredClone(document), id:"legacy-one", name:"旧主题一"},
    {...structuredClone(document), id:"legacy-two", name:"Legacy two"},
  ];
  saved[1].styles.dark["future-author-token"] = "kept raw author value";
  const text = JSON.stringify({envelope_version:1, draft:document, saved_themes:saved, preferences:{...before.preferences, renderer:"dom"}}, null, 2) + "\n";
  await page.evaluate(({key, legacyKey, text}) => {
    window.__theme_studio.dispose();
    localStorage.removeItem(key);
    localStorage.setItem(legacyKey, text);
  }, {key, legacyKey, text});
  await page.reload(); await ready(page);
  return text;
}
async function confirmUpgrade(page:Page) {
  await page.getByTestId("upgrade-review-open").click();
  await expect(page.getByTestId("upgrade-review")).toBeVisible();
  await page.getByTestId("upgrade-confirm").click();
}
async function cacheTools(page:Page) {
  const summary = page.getByText("Cache and original backups", {exact:true});
  if (!(await page.getByTestId("storage-clear-review").isVisible())) await summary.click();
}

test("queued autosave revalidates a changed rem after a real lock wait and remains retryable", async ({page}, info) => {
  await waitForReady(page);
  await page.getByTestId("renderer-dom").click(); await idle(page);
  const errors:string[] = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("console", message => {if (message.type() === "error") errors.push(message.text());});
  page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
  const locker = await second(page); await hold(locker);
  await edit(page, "#223344");
  expect(await page.evaluate(() => window.__theme_studio.edit("radius", "200rem"))).toBe(true);
  const waiting = await snapshot(page);
  expect(waiting.storage.pending).toBe(2);
  await page.evaluate(() => {
    document.documentElement.style.fontSize = "32px";
    window.__theme_studio.environment();
  });
  await release(locker);
  await expect.poll(async () => (await snapshot(page)).storage.paused).toBe(true);
  const retained = await snapshot(page), validRaw = (await raw(page))!;
  expect(retained.document).toEqual(waiting.document);
  expect(retained.storage.pending).toBe(1);
  expect(retained.storage.outcomes.at(-1).state).toBe("failed");
  expect(JSON.parse(validRaw).draft.overrides.light.primary).toBe("#223344");
  expect(validRaw).not.toContain("200rem");
  await expect(page.getByTestId("storage-status")).not.toHaveText("Saved on this device");
  await page.getByTestId("storage-retry").click();
  expect((await snapshot(page)).storage.paused).toBe(true);
  expect(await raw(page)).toBe(validRaw);
  const reopened = await second(page);
  await reopened.evaluate(() => {
    window.__theme_studio.dispose();
    document.documentElement.style.fontSize = "32px";
    window.__theme_studio.mount();
  });
  expect((await snapshot(reopened)).storage.permission).toBe("Ready");
  expect((await snapshot(reopened)).values.light.primary).toBe("#223344");
  await reopened.close();
  await page.evaluate(() => {
    document.documentElement.style.removeProperty("font-size");
    window.__theme_studio.environment();
  });
  await page.getByTestId("storage-retry").click(); await idle(page);
  expect((await snapshot(page)).document).toEqual(waiting.document);
  const stored = await raw(page);
  expect(stored).toContain("200rem");
  await locker.close(); await page.reload(); await ready(page);
  expect((await snapshot(page)).document).toEqual(waiting.document);
  await expect(page.getByTestId("storage-status")).toHaveText("Saved on this device");
  await page.getByTestId("storage-status").screenshot({path:info.outputPath("retry-restored-status.png")});
  expect(errors).toEqual([]);
});

test("valid v2 reload reports the restored draft as saved while a new library stays unconfirmed", async ({page}, info) => {
  await waitForReady(page);
  expect((await snapshot(page)).storage.acknowledged).toBeNull();
  expect(await raw(page)).toBeNull();
  await expect(page.getByTestId("storage-status")).toContainText("no current save has been confirmed");
  await page.getByTestId("renderer-dom").click();
  await save(page, "Restored saved draft"); await idle(page);
  const saved = await snapshot(page), stored = await raw(page);
  await page.reload(); await ready(page);
  const restored = await snapshot(page);
  expect(restored.document).toEqual(saved.document);
  expect(restored.storage.pending).toBe(0);
  expect(restored.storage.acknowledged).toEqual({epoch:0, document:restored.document, preferences:restored.preferences});
  expect(await raw(page)).toBe(stored);
  await expect(page.getByTestId("storage-status")).toHaveText("Saved on this device");
  await page.getByTestId("storage-status").screenshot({path:info.outputPath("restored-status.png")});
});

test("one writer preserves named action order behind a real lock and eventually saves the newest draft and preferences", async ({page}) => {
  await waitForReady(page);
  await page.getByTestId("renderer-dom").click();
  await edit(page, "#112233"); await idle(page);
  const locker = await second(page); await hold(locker);
  await edit(page, "#223344");
  await save(page, "Queued one");
  await edit(page, "#334455");
  await save(page, "Queued two");
  await edit(page, "#445566");
  await page.getByTestId("mode-dark").click();
  await page.getByTestId("locale-zh").click();
  const waiting = await snapshot(page);
  expect(waiting.storage.pending).toBeGreaterThan(2);
  expect(waiting.storage.acknowledged.epoch).not.toBe(waiting.editor_epoch);
  await page.getByTestId("theme-export").click();
  expect(await page.getByTestId("export-source").inputValue()).toContain("#445566");
  await page.keyboard.press("Escape");
  await release(locker); await idle(page);
  const current = await snapshot(page), stored = JSON.parse((await raw(page))!);
  expect(current.document).toEqual(waiting.document);
  expect(current.checkpoint).toEqual(waiting.checkpoint);
  expect(current.history).toEqual(waiting.history);
  expect(current.mode).toBe("dark"); expect(current.preferences.locale).toBe("zh");
  expect(stored.saved_themes.map((theme:any) => theme.name)).toEqual(["Queued one", "Queued two"]);
  expect(stored.saved_themes[0].overrides.light.primary).toBe("#223344");
  expect(stored.saved_themes[1].overrides.light.primary).toBe("#334455");
  expect(stored.draft.overrides.light.primary).toBe("#445566");
  expect(stored.preferences.mode).toBe("dark"); expect(stored.preferences.locale).toBe("zh");
  expect(current.storage.outcomes.filter((entry:any) => entry.kind === "save").map((entry:any) => entry.state)).toEqual(["saved", "saved"]);
  expect(current.storage.paused).toBe(false);
  await locker.close(); await page.reload(); await ready(page);
  expect((await snapshot(page)).document).toEqual(current.document);
  expect((await snapshot(page)).history).toEqual({undo:0, redo:0, gesture:false});
});

test("an older named acknowledgement during a live gesture cannot replace identity, history, checkpoint or unsaved status", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await idle(page);
  const locker = await second(page); await hold(locker);
  await save(page, "Older name");
  const plane = page.getByTestId("color-plane"); await plane.scrollIntoViewIfNeeded();
  const box = (await plane.boundingBox())!;
  await page.mouse.move(box.x + box.width * .2, box.y + box.height * .3); await page.mouse.down();
  await page.mouse.move(box.x + box.width * .65, box.y + box.height * .55);
  const gesturing = await snapshot(page); expect(gesturing.history.gesture).toBe(true);
  await release(locker);
  await expect.poll(async () => (await snapshot(page)).storage.pending).toBe(0);
  const acknowledged = await snapshot(page);
  expect(acknowledged.document).toEqual(gesturing.document);
  expect(acknowledged.checkpoint).toEqual(gesturing.checkpoint);
  expect(acknowledged.history).toEqual(gesturing.history);
  expect(acknowledged.storage.acknowledged.epoch).not.toBe(acknowledged.editor_epoch);
  await expect(page.getByTestId("storage-status")).not.toHaveText("Saved on this device");
  await page.mouse.up(); await idle(page);
  const committed = await snapshot(page);
  const finalSample = structuredClone(gesturing.document);
  // Pointerup samples the final coordinates against the current plane bounds.
  // The saved-operation feedback can move those bounds while the pointer is held.
  finalSample.overrides.light.primary = committed.document.overrides.light.primary;
  expect(committed.document).toEqual(finalSample);
  expect(committed.checkpoint).toEqual(gesturing.checkpoint);
  expect(committed.storage.acknowledged.document).toEqual(committed.document);
  expect(committed.document.name).not.toBe("Older name");
  expect(committed.history.undo).toBe(gesturing.history.undo + 1);
  expect(JSON.parse((await raw(page))!).saved_themes[0].name).toBe("Older name");
  await locker.close();
});

test("cancelled legacy upgrade stays write-protected through edits, history, mode, preferences and reopening", async ({page}) => {
  const original = await seedLegacy(page);
  expect((await snapshot(page)).storage.acknowledged).toBeNull();
  await expect(page.getByTestId("storage-status")).not.toHaveText("Saved on this device");
  await expect(page.getByTestId("preset-count")).toHaveText("46");
  expect((await snapshot(page)).document.id).toBe("legacy-draft");
  expect(await raw(page)).toBeNull(); expect(await backups(page)).toEqual({});
  await page.getByTestId("upgrade-review-open").click();
  const candidate = JSON.parse(await page.getByTestId("upgrade-candidate").inputValue());
  expect(candidate.saved_themes.map((theme:any) => theme.id)).toEqual(["legacy-one", "legacy-two"]);
  await page.getByTestId("upgrade-cancel").click();
  await edit(page, "#234567");
  await page.getByTestId("theme-undo").click(); await page.getByTestId("theme-redo").click();
  await page.getByTestId("mode-dark").click(); await page.getByTestId("preset-favorite").click();
  await save(page, "Memory only");
  await page.getByTestId("locale-zh").click();
  expect((await snapshot(page)).storage.permission).toBe("UpgradeCancelled");
  expect((await snapshot(page)).storage.acknowledged).toBeNull();
  expect(await raw(page)).toBeNull(); expect(await backups(page)).toEqual({});
  expect(await raw(page, legacyKey)).toBe(original);
  await page.getByTestId("theme-export").click(); expect(await page.getByTestId("export-source").inputValue()).toContain("#234567");
  await page.keyboard.press("Escape");
  expect(await download(page, "legacy-download")).toBe(original);
  await page.reload(); await ready(page);
  expect((await snapshot(page)).values.light.primary).not.toBe("#234567");
  expect((await snapshot(page)).storage.permission).toBe("UpgradePending");
  expect((await snapshot(page)).storage.acknowledged).toBeNull();
  expect(await raw(page, legacyKey)).toBe(original); expect(await raw(page)).toBeNull();
});

test("upgrade review is bound to editor epoch and commits only after refreshed explicit confirmation", async ({page}) => {
  const original = await seedLegacy(page);
  await page.getByTestId("upgrade-review-open").click();
  const before = await snapshot(page);
  await edit(page, "#abcdef"); await page.getByTestId("mode-dark").click();
  await expect(page.getByTestId("upgrade-review-status")).toHaveAttribute("data-stale", "true");
  await page.getByTestId("upgrade-confirm").click();
  expect(await raw(page)).toBeNull(); expect(await backups(page)).toEqual({});
  await expect(page.getByTestId("field-error")).toContainText("confirm the new snapshot");
  const reviewed = await snapshot(page);
  await page.getByTestId("upgrade-confirm").click(); await idle(page);
  const stored = JSON.parse((await raw(page))!);
  expect(stored.envelope_version).toBe(2); expect(stored.revision).toBe(1);
  expect(stored.migrated_from_v1).toEqual({sha256:createHash("sha256").update(original).digest("hex"), backup_key:backupKey(original)});
  expect(await raw(page, backupKey(original))).toBe(original);
  expect(await raw(page, legacyKey)).toBe(original);
  expect(stored.saved_themes).toHaveLength(2);
  expect((await snapshot(page)).history).toEqual(reviewed.history);
  expect((await snapshot(page)).mode).toBe("dark");
  expect((await snapshot(page)).document).toEqual(reviewed.document);
  expect(reviewed.history.undo).toBe(before.history.undo + 1);
});

test("a legacy writer changing reviewed bytes while the upgrade waits for a real lock forces a new review", async ({page}) => {
  const original = await seedLegacy(page);
  const locker = await second(page); await hold(locker);
  await confirmUpgrade(page);
  const changed = JSON.parse(original); changed.saved_themes[1].name = "Changed by old window";
  const changedRaw = JSON.stringify(changed, null, 4);
  await locker.evaluate(({legacyKey, changedRaw}) => localStorage.setItem(legacyKey, changedRaw), {legacyKey, changedRaw});
  await expect(page.getByTestId("legacy-changed")).toBeVisible();
  await release(locker);
  await expect.poll(async () => (await snapshot(page)).storage.paused).toBe(true);
  expect(await raw(page)).toBeNull(); expect(await backups(page)).toEqual({});
  await page.getByTestId("upgrade-review-open").click();
  expect(JSON.parse(await page.getByTestId("upgrade-candidate").inputValue()).saved_themes[1].name).toBe("Changed by old window");
  await page.getByTestId("upgrade-confirm").click(); await idle(page);
  expect(await raw(page, backupKey(changedRaw))).toBe(changedRaw);
  expect(await raw(page, backupKey(original))).toBeNull();
  await locker.close();
});

for (const failure of ["backup-write", "backup-readback", "v2-write"] as const) {
  test(`migration ${failure} failure never claims a v2 save and explicit retry is idempotent`, async ({page}) => {
    const original = await seedLegacy(page);
    await page.evaluate(({failure, key, prefix}) => {
      const owner = window as any;
      owner.__originalSet = Storage.prototype.setItem; owner.__originalGet = Storage.prototype.getItem;
      Storage.prototype.setItem = function (name, value) {
        if (failure === "backup-write" && name.startsWith(prefix) || failure === "v2-write" && name === key) throw new DOMException("Injected capacity failure", "QuotaExceededError");
        return owner.__originalSet.call(this, name, value);
      };
      Storage.prototype.getItem = function (name) {
        const value = owner.__originalGet.call(this, name);
        return failure === "backup-readback" && name.startsWith(prefix) && value !== null ? "injected wrong read-back" : value;
      };
    }, {failure, key, prefix});
    await confirmUpgrade(page);
    await expect(page.getByTestId("storage-issue")).toBeVisible();
    expect(await raw(page)).toBeNull(); expect(await raw(page, legacyKey)).toBe(original);
    await expect(page.getByTestId("storage-status")).not.toHaveText("Saved on this device");
    await page.evaluate(() => {
      Storage.prototype.setItem = (window as any).__originalSet;
      Storage.prototype.getItem = (window as any).__originalGet;
    });
    await page.getByTestId("storage-retry").click();
    await expect(page.getByTestId("upgrade-review")).toBeVisible();
    expect(await raw(page)).toBeNull();
    await page.getByTestId("upgrade-confirm").click(); await idle(page);
    expect(await backups(page)).toEqual({[backupKey(original)]:original});
    expect(await raw(page, legacyKey)).toBe(original);
    expect(JSON.parse((await raw(page))!).revision).toBe(1);
  });
}

test("an immutable backup collision is preserved instead of overwritten during migration", async ({page}) => {
  const original = await seedLegacy(page);
  const name = backupKey(original), conflicting = "pre-existing different bytes";
  await page.evaluate(({name, conflicting}) => localStorage.setItem(name, conflicting), {name, conflicting});
  await confirmUpgrade(page); await expect(page.getByTestId("storage-issue")).toContainText("different bytes");
  expect(await raw(page)).toBeNull(); expect(await raw(page, name)).toBe(conflicting);
  expect(await raw(page, legacyKey)).toBe(original);
});

test("invalid v2 protects its exact bytes without falling back to a valid legacy library", async ({page}) => {
  const original = await seedLegacy(page), broken = '{"envelope_version":99,"retained":"坏 v2 原文"}';
  await page.evaluate(({key, broken}) => localStorage.setItem(key, broken), {key, broken});
  await page.reload(); await ready(page);
  await expect(page.getByTestId("corrupt-cache")).toBeVisible();
  await expect(page.getByTestId("upgrade-notice")).toHaveCount(0);
  expect((await snapshot(page)).storage.acknowledged).toBeNull();
  await expect(page.getByTestId("storage-status")).not.toHaveText("Saved on this device");
  expect((await snapshot(page)).document.id).not.toBe("legacy-draft");
  await edit(page, "#123456"); await page.getByTestId("mode-dark").click();
  expect(await raw(page)).toBe(broken); expect(await raw(page, legacyKey)).toBe(original);
  expect(await download(page, "cache-download")).toBe(broken);
});

for (const invalid of ["unknown-envelope", "bad-saved-item", "unknown-document"] as const) {
  test(`legacy ${invalid} rejects the complete migration and retains downloadable original bytes`, async ({page}) => {
    const original = await seedLegacy(page), value = JSON.parse(original);
    if (invalid === "unknown-envelope") value.envelope_version = 99;
    if (invalid === "bad-saved-item") value.saved_themes[1].styles.dark.primary = "var(--unsafe)";
    if (invalid === "unknown-document") value.saved_themes[1].schema_version = 99;
    const broken = JSON.stringify(value, null, 2);
    await page.evaluate(({legacyKey, broken}) => localStorage.setItem(legacyKey, broken), {legacyKey, broken});
    await page.reload(); await ready(page);
    await expect(page.getByTestId("corrupt-cache")).toBeVisible();
    expect((await snapshot(page)).storage.permission).toBe("Protected");
    expect(await raw(page)).toBeNull(); expect(await backups(page)).toEqual({});
    await page.getByTestId("theme-library").click(); await expect(page.getByTestId("library-empty")).toBeVisible();
    await page.keyboard.press("Escape");
    expect(await download(page, "cache-download")).toBe(broken);
  });
}

test("a quota failure retains named requests and later edits until an explicit retry saves their snapshots in order", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const previous = await raw(page);
  await page.evaluate(() => {
    (window as any).__originalSet = Storage.prototype.setItem;
    Storage.prototype.setItem = function () { throw new DOMException("Injected full device", "QuotaExceededError"); };
  });
  await save(page, "Retained name"); await expect(page.getByTestId("storage-issue")).toBeVisible();
  await edit(page, "#654321");
  await page.getByTestId("storage-operations").locator("summary").click();
  await expect(page.getByTestId("storage-operations")).toContainText("Retained name");
  expect(await raw(page)).toBe(previous);
  const unsaved = await snapshot(page); expect(unsaved.storage.paused).toBe(true);
  await page.evaluate(() => { Storage.prototype.setItem = (window as any).__originalSet; });
  await page.getByTestId("storage-retry").click(); await idle(page);
  const stored = JSON.parse((await raw(page))!);
  expect(stored.saved_themes[0].name).toBe("Retained name");
  expect(stored.saved_themes[0].overrides.light.primary).toBe("#123456");
  expect(stored.draft.overrides.light.primary).toBe("#654321");
  expect((await snapshot(page)).document).toEqual(unsaved.document);
  expect((await snapshot(page)).checkpoint).toEqual(unsaved.checkpoint);
  await expect(page.getByTestId("storage-issue")).toHaveCount(0);
});

test("clear uses the reviewed v2 baseline, cancels delayed writes and keeps memory, legacy bytes and backups", async ({page}) => {
  const original = await seedLegacy(page); await confirmUpgrade(page); await idle(page);
  const locker = await second(page); await hold(locker);
  await edit(page, "#225577"); await save(page, "Must not revive");
  await cacheTools(page); await page.getByTestId("storage-clear-review").click();
  await page.getByTestId("storage-clear-confirm").click();
  await edit(page, "#336688");
  const retained = await snapshot(page);
  await release(locker);
  await expect.poll(async () => (await snapshot(page)).storage.permission).toBe("Cleared");
  await expect.poll(async () => (await snapshot(page)).storage.pending).toBe(0);
  expect(await raw(page)).toBeNull(); expect(await raw(page, legacyKey)).toBe(original);
  expect(await raw(page, backupKey(original))).toBe(original);
  expect((await snapshot(page)).document).toEqual(retained.document);
  expect((await snapshot(page)).history).toEqual(retained.history);
  await page.getByTestId("mode-dark").click(); expect(await raw(page)).toBeNull();
  await page.getByTestId("storage-retry").click(); await idle(page);
  expect(JSON.parse((await raw(page))!).saved_themes).toEqual([]);
  expect(JSON.parse((await raw(page))!).draft.overrides.light.primary).toBe("#336688");
  await locker.close();
});

test("a third write after clear review requires another decision instead of clearing unreviewed bytes", async ({page}) => {
  await waitForReady(page); await edit(page, "#123456"); await idle(page);
  const other = await second(page); await hold(other);
  await cacheTools(page); await page.getByTestId("storage-clear-review").click();
  await page.getByTestId("storage-clear-confirm").click();
  const intervening = JSON.parse((await raw(page))!); intervening.revision += 1; intervening.preferences.scene = "Mail";
  const newer = JSON.stringify(intervening);
  await other.evaluate(({key, newer}) => localStorage.setItem(key, newer), {key, newer});
  await release(other);
  await expect(page.getByTestId("storage-conflict")).toBeVisible();
  expect(await raw(page)).toBe(newer);
  expect((await snapshot(page)).values.light.primary).toBe("#123456");
  await other.close();
});

for (const decision of ["load", "keep", "copy"] as const) {
  test(`${decision} rechecks reviewed bytes under the lock and a third write requires another explicit decision`, async ({page}) => {
    await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
    const other = await second(page);
    await edit(other, "#654321"); await idle(other);
    await expect(page.getByTestId("storage-conflict")).toBeVisible();
    const displayed = JSON.parse((await raw(page))!);
    await expect(page.getByTestId("storage-external-summary")).toHaveAttribute("data-external-revision", String(displayed.revision));
    await expect(page.getByTestId("storage-external-summary")).toContainText(displayed.draft.name);
    const current = await snapshot(page);
    await hold(other);
    async function decide() {
      await page.getByTestId(`conflict-${decision}`).click();
      if (decision === "load") await page.getByTestId("theme-confirm").click();
      if (decision === "copy") {
        await page.getByTestId("theme-name").fill("Retained branch");
        await page.getByTestId("theme-name-confirm").click();
      }
    }
    await decide();
    const third = JSON.parse((await raw(page))!);
    third.revision += 1;
    third.draft.overrides.light.primary = "#abcdef";
    const saved = structuredClone(third.draft); saved.id = "third-theme"; saved.name = "Third library entry";
    third.saved_themes.push(saved);
    const thirdRaw = JSON.stringify(third);
    // This represents an old, non-cooperating window. The lock is real; the
    // extra write intentionally bypasses its protocol to exercise comparison.
    await other.evaluate(({key, thirdRaw}) => localStorage.setItem(key, thirdRaw), {key, thirdRaw});
    await release(other);
    await expect(page.getByTestId("storage-conflict")).toBeVisible();
    await expect.poll(async () => (await snapshot(page)).storage.paused).toBe(true);
    await expect(page.getByTestId("storage-external-summary")).toHaveAttribute("data-external-revision", String(third.revision));
    await page.getByTestId("storage-external-raw").locator("summary").click();
    expect(await page.getByTestId("storage-external-source").inputValue()).toBe(thirdRaw);
    expect(await raw(page)).toBe(thirdRaw);
    expect((await snapshot(page)).document).toEqual(current.document);
    expect((await snapshot(page)).history).toEqual(current.history);
    await decide();
    await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
    await idle(page);
    const resolved = await snapshot(page), stored = JSON.parse((await raw(page))!);
    expect(stored.saved_themes.some((theme:any) => theme.id === "third-theme")).toBe(true);
    if (decision === "load") expect(resolved.values.light.primary).toBe("#abcdef");
    else expect(resolved.values.light.primary).toBe("#123456");
    if (decision === "copy") {
      expect(stored.saved_themes.filter((theme:any) => theme.name === "Retained branch")).toHaveLength(1);
      expect(resolved.document.name).toBe("Retained branch");
    }
    await other.close();
  });
}

test("Keep approves only its displayed draft while retained named intents require a separate retry or cancellation", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const other = await second(page); await hold(other);
  await edit(page, "#234567"); await save(page, "Retained pending name");
  const external = JSON.parse((await raw(page))!); external.revision += 1; external.draft.overrides.light.primary = "#abcdef";
  await other.evaluate(({key, value}) => localStorage.setItem(key, value), {key, value:JSON.stringify(external)});
  await expect(page.getByTestId("storage-conflict")).toBeVisible(); await release(other);
  await page.getByTestId("conflict-keep").click();
  await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  await expect.poll(async () => (await snapshot(page)).storage.paused).toBe(true);
  expect(JSON.parse((await raw(page))!).saved_themes).toEqual([]);
  expect((await snapshot(page)).storage.pending).toBeGreaterThan(0);
  await page.getByTestId("storage-operations").locator("summary").click();
  await expect(page.getByTestId("storage-operations")).toContainText("Retained pending name");
  await page.getByTestId("storage-cancel").click();
  expect((await snapshot(page)).storage.pending).toBe(0);
  expect(JSON.parse((await raw(page))!).saved_themes).toEqual([]);
  expect((await snapshot(page)).storage.outcomes.some((entry:any) => entry.detail === "Retained pending name" && entry.state === "cancelled")).toBe(true);
  await other.close();
});

test("backup deletion compares its reviewed full key set inside the same lock and ordinary clear leaves every original", async ({page}) => {
  const original = await seedLegacy(page); await confirmUpgrade(page); await idle(page);
  const other = await second(page), additional = original + " ";
  await hold(other); await cacheTools(page); await page.getByTestId("backups-review").click();
  await page.getByTestId("backups-clear-confirm").click();
  await other.evaluate(({name, additional}) => localStorage.setItem(name, additional), {name:backupKey(additional), additional});
  await release(other);
  await expect.poll(async () => (await snapshot(page)).storage.paused).toBe(true);
  expect(await backups(page)).toEqual({[backupKey(original)]:original, [backupKey(additional)]:additional});
  await page.getByTestId("storage-retry").click();
  await expect(page.getByTestId("backup-download")).toHaveCount(2);
  await page.getByTestId("backups-clear-confirm").click();
  await expect.poll(() => backups(page)).toEqual({});
  expect(await raw(page, legacyKey)).toBe(original);
  await other.close();
});

test("owner disposal aborts a real pending lock request and cannot write or call destroyed signals after grant", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const before = await raw(page), other = await second(page); await hold(other);
  const errors:string[] = []; page.on("pageerror", error => errors.push(error.message));
  await edit(page, "#abcdef");
  await expect.poll(async () => (await snapshot(page)).storage.active).toBe(true);
  expect(await page.evaluate(() => window.__theme_studio.dispose())).toBe(true);
  await release(other);
  await expect.poll(() => page.evaluate(async name => (await navigator.locks.query()).pending?.filter(entry => entry.name === name).length, lock)).toBe(0);
  expect(await raw(page)).toBe(before); expect(errors).toEqual([]);
  await page.evaluate(() => window.__theme_studio.mount());
  expect((await snapshot(page)).values.light.primary).toBe("#123456");
  await other.close();
});

test("the host fatal boundary aborts a real pending write and restart restores only the confirmed draft", async ({page}) => {
  test.setTimeout(90_000);
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const before = await raw(page), saved = (await snapshot(page)).document;
  const previous = await page.evaluate(() => window.__theme_studio.diagnostics());
  const other = await second(page); await hold(other);
  const errors:string[] = []; page.on("pageerror", error => errors.push(error.message));
  await edit(page, "#abcdef");
  await expect.poll(() => page.evaluate(async name => (await navigator.locks.query()).pending?.filter(entry => entry.name === name).length, lock)).toBe(1);
  expect((await snapshot(page)).document).not.toEqual(saved);
  // Exercise the real loader recovery boundary; the Studio has no deliberate
  // WASM trap export. The unconfirmed editor value is intentionally discarded.
  await page.evaluate(() => {
    (window as any).__fatalStorageOwner = window.__theme_studio;
    (window.__theme_studio as any).hooks.runtime.enter_fatal(new Error("Pending storage recovery check"));
  });
  await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
  await expect(page.getByTestId("fatal-text")).toContainText("restart to restore the last saved draft");
  expect(await page.evaluate(() => typeof window.__theme_studio)).toBe("undefined");
  expect(await page.evaluate(() => (window as any).__fatalStorageOwner.hooks.signal.aborted)).toBe(true);
  await expect.poll(() => page.evaluate(async name => (await navigator.locks.query()).pending?.filter(entry => entry.name === name).length, lock)).toBe(0);
  await release(other);
  expect(await raw(page)).toBe(before); expect(errors).toEqual([]);
  await other.close();
  await page.getByTestId("fatal-restart").click(); await ready(page);
  const restored = await snapshot(page);
  expect(restored.document).toEqual(saved);
  expect(restored.values.light.primary).toBe("#123456");
  expect(restored.storage).toMatchObject({permission:"Ready", pending:0, paused:false});
  expect(await raw(page)).toBe(before);
  const next = await page.evaluate(() => window.__theme_studio.diagnostics());
  expect(next.runtime).toBe(previous.runtime + 1); expect(next.build).toBe(previous.build);
  expect(errors).toEqual([]);
});

for (const focused of ["parameter", "hsl"] as const) {
  test(`owner disposal ignores late ${focused} blur while a real storage lock is pending`, async ({page}) => {
    await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
    const before = await raw(page), other = await second(page); await hold(other);
    const errors:string[] = []; page.on("pageerror", error => errors.push(error.message));
    await edit(page, "#abcdef");
    await page.getByTestId("editor-controls").click();
    if (focused === "parameter") {
      await page.getByTestId("edit-radius").fill("8px"); await page.getByTestId("edit-radius").press("Enter");
    } else await page.getByTestId("hsl-hue").fill("30");
    expect(await page.evaluate(() => window.__theme_studio.dispose())).toBe(true);
    await release(other);
    await expect.poll(() => page.evaluate(async name => (await navigator.locks.query()).pending?.filter(entry => entry.name === name).length, lock)).toBe(0);
    expect(await raw(page)).toBe(before); expect(errors).toEqual([]);
    await other.close();
  });
}

test("without Web Locks the editor, undo and downloads work while library actions never claim a save", async ({page}) => {
  await page.addInitScript(() => Object.defineProperty(navigator, "locks", {value:undefined, configurable:true}));
  await waitForReady(page);
  await expect(page.getByTestId("storage-status")).toContainText("memory editing");
  await edit(page, "#789abc"); await save(page, "Not persisted");
  await expect(page.getByTestId("field-error")).toContainText("Local writing is unavailable");
  await page.getByTestId("storage-operations").locator("summary").click();
  await expect(page.getByTestId("storage-operations").locator('li[data-op-state="rejected"]')).toContainText("Not persisted");
  expect(await raw(page)).toBeNull();
  await page.getByTestId("theme-undo").click(); await page.getByTestId("theme-redo").click();
  await page.getByTestId("theme-export").click();
  expect(await page.getByTestId("export-source").inputValue()).toContain("#789abc");
  expect(await download(page, "export-download")).toContain("#789abc");
  await expect(page.getByTestId("storage-status")).not.toHaveText("Saved on this device");
});

test("reopening retrieves the SHA-bound original even after an older window changes v1, and a damaged backup is never substituted", async ({page}) => {
  const original = await seedLegacy(page); await confirmUpgrade(page); await idle(page);
  await page.reload(); await ready(page);
  expect(await download(page, "legacy-download")).toBe(original);
  const before = await raw(page), other = await second(page), changed = original + "\n ";
  await other.evaluate(({legacyKey, changed}) => localStorage.setItem(legacyKey, changed), {legacyKey, changed});
  await expect(page.getByTestId("legacy-changed")).toBeVisible();
  expect(await raw(page)).toBe(before);
  expect(await download(page, "legacy-download")).toBe(original);
  expect(await download(page, "legacy-changed-download")).toBe(changed);
  await page.reload(); await ready(page); await expect(page.getByTestId("legacy-changed")).toBeVisible();
  await other.evaluate(name => localStorage.setItem(name, "damaged backup"), backupKey(original));
  await page.getByTestId("legacy-download").click();
  await expect(page.getByTestId("field-error")).toContainText("digest does not match");
  expect(await raw(page)).toBe(before); expect(await raw(page, legacyKey)).toBe(changed);
  await other.close();
});

test("delayed Save and Delete intents keep their order across undo/redo and finish with the latest committed draft", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click();
  await save(page, "Disposable"); await idle(page);
  const disposable = JSON.parse((await raw(page))!).saved_themes[0].id;
  await page.getByTestId("preset-select").click();
  await page.getByRole("option", {name:"Built-in · Rustify Light", exact:true}).click(); await idle(page);
  const other = await second(page); await hold(other);
  await edit(page, "#123456"); await save(page, "First named snapshot");
  await page.getByTestId("theme-library").click();
  await page.locator(`li[data-theme-id="${disposable}"]`).getByTestId("local-delete").click();
  await page.getByTestId("theme-confirm").click(); await page.keyboard.press("Escape");
  await edit(page, "#234567"); await page.getByTestId("theme-undo").click(); await page.getByTestId("theme-redo").click();
  await save(page, "Second named snapshot"); await edit(page, "#345678");
  const waiting = await snapshot(page);
  await release(other); await idle(page);
  const current = await snapshot(page), stored = JSON.parse((await raw(page))!);
  expect(stored.saved_themes.map((theme:any) => theme.name)).toEqual(["First named snapshot", "Second named snapshot"]);
  expect(stored.draft.overrides.light.primary).toBe("#345678");
  expect(current.document).toEqual(waiting.document); expect(current.history).toEqual(waiting.history); expect(current.checkpoint).toEqual(waiting.checkpoint);
  const ordered = current.storage.outcomes.filter((entry:any) => entry.kind === "save" || entry.kind === "delete");
  expect(ordered.map((entry:any) => [entry.kind, entry.state])).toEqual([["save", "saved"], ["save", "saved"], ["delete", "saved"], ["save", "saved"]]);
  await other.close();
});

test("a failed clear reports a retained removal instead of success, and cancellation restores the previous write gate", async ({page}) => {
  const original = await seedLegacy(page); await confirmUpgrade(page); await idle(page);
  const previous = await raw(page);
  await page.evaluate(name => {
    const owner = window as any; owner.__originalRemove = Storage.prototype.removeItem;
    Storage.prototype.removeItem = function (key) {if (key === name) throw new DOMException("Injected removal denied", "SecurityError");return owner.__originalRemove.call(this, key);};
  }, key);
  await cacheTools(page); await page.getByTestId("storage-clear-review").click(); await page.getByTestId("storage-clear-confirm").click();
  await expect(page.getByTestId("storage-issue")).toContainText("removal denied");
  expect((await snapshot(page)).storage.permission).toBe("Clearing");
  await expect(page.getByTestId("storage-status")).not.toContainText("cache cleared");
  expect(await raw(page)).toBe(previous); expect(await raw(page, backupKey(original))).toBe(original);
  await edit(page, "#123456"); await save(page, "Rejected while clearing");
  expect((await snapshot(page)).storage.outcomes.some((entry:any) => entry.detail === "Rejected while clearing" && entry.state === "rejected")).toBe(true);
  await page.getByTestId("storage-operations").locator("summary").click();
  await expect(page.getByTestId("storage-operations").locator('li[data-op-state="rejected"]')).toContainText("Rejected while clearing");
  expect(await raw(page)).toBe(previous);
  await page.getByTestId("storage-cancel").click();
  expect((await snapshot(page)).storage.permission).toBe("Ready");
  await page.evaluate(() => {Storage.prototype.removeItem = (window as any).__originalRemove;});
  await page.getByTestId("storage-retry").click(); await idle(page);
  expect(JSON.parse((await raw(page))!).draft.overrides.light.primary).toBe("#123456");
  expect(await raw(page, legacyKey)).toBe(original);
});

test("the native guard also prevents writes when disposal occurs just after a real lock is granted", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const previous = await raw(page), errors:string[] = []; page.on("pageerror", error => errors.push(error.message));
  await page.evaluate(name => {
    const request = navigator.locks.request.bind(navigator.locks);
    (navigator.locks as any).request = (lockName:string, options:any, callback:any) => request(lockName, options, held => {
      if (lockName === name) { (window as any).__disposedOnGrant = window.__theme_studio.dispose(); }
      return callback(held);
    });
  }, lock);
  await page.getByTestId("color-value").fill("#abcdef"); await page.getByTestId("color-value").press("Enter");
  await expect.poll(() => page.evaluate(() => (window as any).__disposedOnGrant)).toBe(true);
  await expect.poll(() => page.evaluate(async name => (await navigator.locks.query()).held?.filter(entry => entry.name === name).length, lock)).toBe(0);
  expect(await raw(page)).toBe(previous); expect(errors).toEqual([]);
});

test("denied lock requests pause without an unlocked fallback and preserve export until explicit retry", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const previous = await raw(page);
  await page.evaluate(() => {
    const owner = window as any; owner.__originalLockRequest = navigator.locks.request;
    (navigator.locks as any).request = () => Promise.reject(new DOMException("Injected lock access denied", "SecurityError"));
  });
  await edit(page, "#abcdef"); await expect(page.getByTestId("storage-issue")).toContainText("lock access denied");
  await save(page, "Retained denied action");
  expect(await raw(page)).toBe(previous); expect((await snapshot(page)).storage.paused).toBe(true);
  await page.getByTestId("theme-export").click(); expect(await page.getByTestId("export-source").inputValue()).toContain("#abcdef");
  await page.keyboard.press("Escape");
  await page.evaluate(() => {(navigator.locks as any).request = (window as any).__originalLockRequest;});
  await page.getByTestId("storage-retry").click(); await idle(page);
  expect(JSON.parse((await raw(page))!).saved_themes[0].name).toBe("Retained denied action");
});

test("re-entering upgrade review aborts its old lock wait and commits only the newly confirmed candidate", async ({page}) => {
  const original = await seedLegacy(page), other = await second(page); await hold(other);
  await confirmUpgrade(page);
  await edit(page, "#234567");
  await page.getByTestId("upgrade-review-open").click();
  expect(await raw(page)).toBeNull(); expect(await backups(page)).toEqual({});
  const reviewed = await snapshot(page);
  await page.getByTestId("upgrade-confirm").click(); await release(other); await idle(page);
  expect(JSON.parse((await raw(page))!).revision).toBe(1);
  expect((await snapshot(page)).document).toEqual(reviewed.document);
  const upgrades = (await snapshot(page)).storage.outcomes.filter((entry:any) => entry.kind === "upgrade");
  expect(upgrades.map((entry:any) => entry.state)).toEqual(["cancelled", "saved"]);
  expect(await backups(page)).toEqual({[backupKey(original)]:original});
  await other.close();
});

test("an empty external v2 is invalid raw, stays downloadable byte-for-byte and requires explicit reviewed clearing", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const other = await second(page);
  await other.evaluate(name => localStorage.setItem(name, ""), key);
  await expect(page.getByTestId("storage-conflict")).toBeVisible();
  await expect(page.getByTestId("storage-external-summary")).toHaveAttribute("data-external-kind", "invalid");
  await expect(page.getByTestId("storage-external-summary")).toContainText("Invalid external v2 library");
  expect(await download(page, "storage-external-download")).toBe("");
  await page.getByTestId("conflict-keep").click();
  await expect(page.getByTestId("field-error")).not.toBeEmpty();
  expect(await raw(page)).toBe(""); expect((await snapshot(page)).storage.paused).toBe(true);
  expect((await snapshot(page)).values.light.primary).toBe("#123456");
  await cacheTools(page); await page.getByTestId("storage-clear-review").click(); await page.getByTestId("storage-clear-confirm").click();
  await expect.poll(async () => (await snapshot(page)).storage.permission).toBe("Cleared");
  expect(await raw(page)).toBeNull();
  await other.close();
});

test("a removed external v2 is shown as absent and Keep explicitly creates a fresh library", async ({page}) => {
  await waitForReady(page); await page.getByTestId("renderer-dom").click(); await edit(page, "#123456"); await idle(page);
  const other = await second(page);
  await other.evaluate(name => localStorage.removeItem(name), key);
  await expect(page.getByTestId("storage-external-summary")).toHaveAttribute("data-external-kind", "removed");
  await expect(page.getByTestId("storage-external-summary")).toContainText("removed the v2 library");
  await expect(page.getByTestId("storage-external-download")).toHaveCount(0);
  await page.getByTestId("conflict-keep").click(); await idle(page);
  expect(JSON.parse((await raw(page))!).draft.overrides.light.primary).toBe("#123456");
  await expect(page.getByTestId("storage-conflict")).toHaveCount(0);
  await other.close();
});

test("removing v1 remains a changed-source notice after reopening while the original backup stays retrievable", async ({page}) => {
  const original = await seedLegacy(page); await confirmUpgrade(page); await idle(page);
  const stored = await raw(page), other = await second(page);
  await other.evaluate(name => localStorage.removeItem(name), legacyKey);
  await expect(page.getByTestId("legacy-changed")).toBeVisible();
  expect(await raw(page)).toBe(stored);
  expect(await download(page, "legacy-download")).toBe(original);
  await other.close(); await page.reload(); await ready(page);
  await expect(page.getByTestId("legacy-changed")).toBeVisible();
  expect(await raw(page, legacyKey)).toBeNull(); expect(await raw(page)).toBe(stored);
  expect(await download(page, "legacy-download")).toBe(original);
});
