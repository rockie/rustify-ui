import { expect, test } from "@playwright/test";
import { anchorRect, geometry, mountGeometry } from "./support";

/// The same three steps the workbench takes, on the other example: pick an
/// object on the region, edit one of its values in a control the DOM owns, and
/// confirm. Nothing reaches the application before the confirm.
test.describe("M6: select, edit, confirm", () => {
    test("the main path from a pointer on the region to a committed value", async ({ page }) => {
        await mountGeometry(page);
        expect(await geometry(page)).toMatchObject({
            selected: null,
            colors: "475467,475467,475467,475467",
            applied: 0,
        });

        // Select: a real click on an anchor the region reported.
        await page.getByTestId("arm-menu").click();
        const anchors = (await geometry(page)).anchors;
        const rect = await anchorRect(page, anchors[2]);
        await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
        await expect.poll(async () => (await geometry(page)).selected).toBe(2);

        // Edit: the menu anchored to it opens a modal over the value.
        await expect(page.getByTestId("geometry-menu")).toHaveCount(1);
        await page.getByTestId("menu-open-dialog").click();
        const field = page.getByTestId("dialog-input");
        await expect(field).toHaveValue("475467");
        await field.fill("12b76a");
        // Typing is not committing: the region still shows the old value.
        expect(await geometry(page)).toMatchObject({
            colors: "475467,475467,475467,475467",
            applied: 0,
        });

        // Confirm: the value reaches the application, and the modal closes.
        await page.getByTestId("dialog-apply").click();
        await expect.poll(async () => (await geometry(page)).applied).toBe(1);
        expect(await geometry(page)).toMatchObject({
            colors: "475467,475467,12b76a,475467",
            dialog: false,
        });
    });

    test("an abandoned edit changes nothing", async ({ page }) => {
        await mountGeometry(page);
        await page.getByTestId("arm-menu").click();
        const anchors = (await geometry(page)).anchors;
        const rect = await anchorRect(page, anchors[1]);
        await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
        await expect.poll(async () => (await geometry(page)).selected).toBe(1);
        await page.getByTestId("menu-open-dialog").click();
        await page.getByTestId("dialog-input").fill("f04438");
        await page.getByTestId("dialog-close").click();
        await expect.poll(async () => (await geometry(page)).dialog).toBe(false);
        expect(await geometry(page)).toMatchObject({
            colors: "475467,475467,475467,475467",
            applied: 0,
        });
    });

    test("a value the application refuses leaves the old one and says why", async ({ page }) => {
        await mountGeometry(page);
        await page.getByTestId("arm-menu").click();
        const anchors = (await geometry(page)).anchors;
        const rect = await anchorRect(page, anchors[0]);
        await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
        await expect.poll(async () => (await geometry(page)).selected).toBe(0);
        await page.getByTestId("menu-open-dialog").click();

        await page.getByTestId("dialog-input").fill("nope");
        await page.getByTestId("dialog-apply").click();
        await expect(page.getByTestId("dialog-error")).toHaveText(
            "a colour is six hexadecimal digits"
        );
        // The modal stays open on a refusal: the value is still being edited.
        expect(await geometry(page)).toMatchObject({
            dialog: true,
            colors: "475467,475467,475467,475467",
            applied: 0,
        });

        // Corrected and confirmed, it goes through and the notice clears.
        await page.getByTestId("dialog-input").fill("#f04438");
        await page.getByTestId("dialog-apply").click();
        await expect.poll(async () => (await geometry(page)).applied).toBe(1);
        expect(await geometry(page)).toMatchObject({
            colors: "f04438,475467,475467,475467",
            dialog: false,
            refusal: null,
        });
    });
});
