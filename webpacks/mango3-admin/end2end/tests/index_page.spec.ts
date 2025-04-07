import { test, expect } from "@playwright/test";
import { expectLoadToComplete, expectRedirectToHomePage, testAsUser } from "../../../js/shared_expects";

test("should redirects to home page when is not logged in", async ({ page }) => {
    await page.goto("/");

    await expectRedirectToHomePage(page);
});

testAsUser("should redirects to home page when is not admin", async ({ page }) => {
    await page.goto("/");

    await expectRedirectToHomePage(page);
});
