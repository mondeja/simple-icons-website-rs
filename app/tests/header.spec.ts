/**
 * Tests for header component.
 **/

import { test, expect, Page } from "@playwright/test";
import { URL } from "./constants";
import * as icons from "simple-icons";

const LINK_TITLES = [
  "Main Repository",
  "npm",
  "Packagist",
  "jsDelivr (Content Delivery Network)",
  "UNPKG (Content Delivery Network)",
  "Open Collective",
  "Legal Disclaimer",
];

const MENU_TITLE = "Menu";
const CLOSE_MENU_TITLE = "Close menu";

async function checkMenuButtons(page: Page, visible: boolean) {
  for (let i = 0; i < LINK_TITLES.length; i++) {
    let link = await page.locator(`li:nth-child(${i + 1}) a`);
    await expect(link).toHaveAttribute("title", LINK_TITLES[i]);
    // Mobile browsers have the links hidden in a burger menu
    await expect(link).toBeVisible({ visible });
  }
}

test("title and description", async ({ page }) => {
  await page.goto(URL);

  await expect(page.locator("h1")).toHaveText("Simple Icons");
  await expect(page.locator("p")).toHaveText(
    `${Object.keys(icons).length - 1} free SVG icons for popular brands`
  );
});

test.describe("menu", async () => {
  test("links", async ({ page, isMobile }) => {
    await page.goto(URL);

    await checkMenuButtons(page, !isMobile);

    // Menu button is hidden
    const burger = await page.locator(`li[title="${MENU_TITLE}"]`);
    await expect(burger).toBeVisible({
      visible: isMobile,
    });
  });

  test("burger menu", async ({ page, isMobile }) => {
    test.skip(!isMobile, "Burger menu only available on mobile screens");

    await page.goto(URL);
    const burger = await page.locator(`li[title="${MENU_TITLE}"]`);
    await expect(burger).toBeVisible();

    // Menu links are hidden
    await checkMenuButtons(page, false);

    // Open burger menu
    await burger.click();

    // Menu links are visible
    await checkMenuButtons(page, true);
    // Burger button is hidden
    await expect(burger).toBeVisible({ visible: false });
    // Title is hidden
    await expect(page.locator("h1")).toBeVisible({ visible: false });

    // Close burger menu
    const close = await page.locator(`li[title="${CLOSE_MENU_TITLE}"]`);
    await close.click();

    // Menu links are hidden
    await checkMenuButtons(page, false);
  });
});
