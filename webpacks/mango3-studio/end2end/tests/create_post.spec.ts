import { faker } from "@faker-js/faker/locale/en";
import path from "path";
import { test, expect } from "@playwright/test";
import { expectLoadToComplete, expectRedirectToLoginPage, testAsUser } from "../../../js/shared_expects";

testAsUser("should create a new post", async ({ page }) => {
    await page.goto("/new-website");

    await expect(page.locator("h2")).toHaveText("New website");

    const websiteName = faker.internet.displayName();

    await page.getByLabel("Name").pressSequentially(websiteName);
    await page.getByLabel("Description").fill(faker.lorem.sentence());
    await page.getByText("Submit").click();

    await expect(page.getByText("Website created successfully")).toBeVisible();

    await page.getByRole("button", { name: "Ok", exact: true }).click();

    await expect(page.locator("h2")).toHaveText("My websites");

    await page
        .locator("div.card", { has: page.getByText(websiteName) })
        .getByRole("link", { name: "Select" })
        .click();

    await page.getByText("Posts").click();
    await page.getByText("New post").click();

    await expect(page.locator("h2")).toHaveText("New post");

    await page.getByLabel("Title").pressSequentially(faker.lorem.sentence());
    await page.getByLabel("Content").fill(faker.lorem.paragraphs());

    const attachedImagesChooserPromise = page.waitForEvent("filechooser");

    await page.getByLabel("Attached images").click();

    const attachedImagesChooser = await attachedImagesChooserPromise;

    await attachedImagesChooser.setFiles(path.join(__dirname, "../../../../assets/favicon.png"));

    const coverImageChooserPromise = page.waitForEvent("filechooser");

    await page.getByLabel("Cover image").click();

    const coverImageChooser = await coverImageChooserPromise;

    await coverImageChooser.setFiles(path.join(__dirname, "../../../../assets/favicon.png"));

    await page.getByText("Submit").click();

    await expect(page.getByText("Post created successfully")).toBeVisible();
});
