import { test, expect } from "@playwright/test";
import { expectLoadToComplete } from "../../../js/shared_expects";

test("should display no results when there is no match", async ({ page }) => {
    await page.goto("/");

    await expectLoadToComplete(page);

    const searchInput = page.locator("input[type=search]").locator("visible=true");

    await searchInput.pressSequentially("Some unexistent website");
    await searchInput.press("Enter");
    await page.getByRole("tab", { name: "Websites" }).click();

    await expect(page).toHaveURL(/\/search\?q=Some%20unexistent%20website&tab=websites$/);
    await expect(page.locator("h1")).toHaveText("Search results for “Some unexistent website”");
    await expect(page.getByText("No results found.")).toBeVisible();
});
