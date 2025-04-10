import { test, expect } from "@playwright/test";
import { expectLoadToComplete } from "../../../js/shared_expects";

test("does not exists", async ({ page }) => {
    await page.goto("/");

    await expectLoadToComplete(page);

    await expect(page).toHaveTitle("Error 404: Page not found | Mango³ Dev");

    await expect(page.locator("h1")).toHaveText("Whoops!");
});
