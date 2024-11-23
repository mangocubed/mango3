import { test, expect } from "@playwright/test";
import { expectLoadToComplete, expectRedirectToLoginPage, mocksLoginPage } from "../../../js/shared_expects";

test("redirects to login page", async ({ page }) => {
    await mocksLoginPage(page);

    await page.goto("/");

    await expectRedirectToLoginPage(page);
});
