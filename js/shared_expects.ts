import { test, expect, Page } from "@playwright/test";

export async function expectLoadToComplete(page: Page) {
    await expect(page.locator(".loading-overlay")).toHaveClass(/is-done/);
}
