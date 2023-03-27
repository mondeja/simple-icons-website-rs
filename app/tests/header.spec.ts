import { test, expect } from "@playwright/test";
import { URL } from "./constants";
import * as icons from "simple-icons";

test("title and description", async ({ page }) => {
  await page.goto(URL);

  await expect(page.locator("h1")).toHaveText("Simple Icons");
  await expect(page.locator("p")).toHaveText(
    `${Object.keys(icons).length - 1} free SVG icons for popular brands`
  );
});
