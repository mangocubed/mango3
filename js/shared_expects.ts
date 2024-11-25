import { test, expect, Page } from "@playwright/test";

const loginPageUrl = "http://accounts.mango3.local/login";

export async function expectLoadToComplete(page: Page) {
    await expect(page.locator(".loading-overlay")).toHaveClass(/is-done/);
}

export async function expectRedirectToLoginPage(page: Page) {
    await expect(page).toHaveURL(loginPageUrl);
}

export async function mocksLoginPage(page: Page) {
    await page.route(loginPageUrl, async route => {
        await route.fullFill({ status: 200, body: "Login" });
    });
}