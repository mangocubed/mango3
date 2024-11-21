import { test, expect } from "@playwright/test";

test("login page has a heading text", async ({ page }) => {
    await page.goto("/login");

    await expect(page.locator("h2")).toHaveText("Login");
});
