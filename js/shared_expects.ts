import { test, expect, Page } from "@playwright/test";
import path from "path";

export const storageFile = path.join(__dirname, "../.end2end-storage.json");

const homePageUrl = "http://mango3.local";
const loginPageUrl = "http://accounts.mango3.local/login";

export async function expectLoadToComplete(page: Page) {
    await expect(page.locator(".loading-overlay")).toHaveClass(/is-done/);
}

export async function expectRedirectToHomePage(page: Page) {
    await expect(page).toHaveURL(homePageUrl);
}

export async function expectRedirectToLoginPage(page: Page) {
    await expect(page).toHaveURL(loginPageUrl);
}

export const testAsUser = test.extend<{}>({
    storageState: ({}, use) => use(storageFile),
});
