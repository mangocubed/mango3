import { test, expect } from "@playwright/test";
import { expectLoadToComplete } from "../../../js/shared_expects";

test("index page has a heading text", async ({ page }) => {
    await page.goto("/");

    await expectLoadToComplete(page);

    await expect(page.locator("h2")).toHaveText("Welcome to Mango³ Dev!");
});
