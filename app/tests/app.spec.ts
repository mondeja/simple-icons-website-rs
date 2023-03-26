import { test, expect } from "@playwright/test";
import { URL } from "./constants";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto(URL);

  await expect(page.locator("h1")).toHaveText("Simple Icons");
});
