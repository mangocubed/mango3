import { test, expect } from "@playwright/test";

test("index page has a heading text", async ({ page }) => {
    await page.goto("/");

    await expect(page.locator("h2")).toHaveText("Welcome to Mango³ Dev!");
});
