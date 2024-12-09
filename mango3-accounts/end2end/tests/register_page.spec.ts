import { faker } from "@faker-js/faker/locale/en";
import { expect, test } from "@playwright/test";
import { expectLoadToComplete, storageFile } from "../../../js/shared_expects";

test("should register a new user", async ({ page }) => {
    await page.goto("/register");

    await expectLoadToComplete(page);

    await expect(page.locator("h2")).toHaveText("Register");

    await page.getByLabel("Username").fill(faker.internet.username());
    await page.getByLabel("Email").fill(faker.internet.email());
    await page.getByLabel("Password").fill(faker.internet.password());
    await page.getByLabel("Full name").fill(faker.person.fullName());
    await page.getByLabel("Birthdate").fill(faker.date.birthdate().toISOString().split("T")[0]);
    await page.getByLabel("Country").selectOption(faker.location.countryCode());
    await page.getByText("Submit").click();

    await expect(page.getByText("User created successfully")).toBeVisible();

    await page.context().storageState({ path: storageFile });
});