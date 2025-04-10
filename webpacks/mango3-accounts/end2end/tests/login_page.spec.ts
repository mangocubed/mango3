import { expect, test } from "@playwright/test";
import { expectLoadToComplete } from "../../../js/shared_expects";

test("should have a heading text", async ({ page }) => {
    await page.goto("/login");

    await expectLoadToComplete(page);

    await expect(page.locator("h1")).toHaveText("Login");
});
