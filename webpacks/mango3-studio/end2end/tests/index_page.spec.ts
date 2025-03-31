import { test, expect } from "@playwright/test";
import { expectLoadToComplete, expectRedirectToLoginPage, testAsUser } from "../../../js/shared_expects";

test("should redirects to login page when is not logged in", async ({ page }) => {
    await page.goto("/");

    await expectRedirectToLoginPage(page);
});

testAsUser("should have a heading text", async ({ page }) => {
    await page.goto("/");

    await expect(page.locator("h2")).toHaveText("My websites");
});
