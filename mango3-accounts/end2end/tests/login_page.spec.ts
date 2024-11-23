import { test, expect } from "@playwright/test";
import { expectLoadToComplete } from "../../../js/shared_expects";

test("has a heading text", async ({ page }) => {
    await page.goto("/login");

    await expectLoadToComplete(page);

    await expect(page.locator("h2")).toHaveText("Login");
});
