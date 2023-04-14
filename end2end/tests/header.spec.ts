/**
 * Tests for header component.
 **/

import { test, expect } from '@playwright/test';
import { minBreakpoint } from '../helpers.ts';
import * as icons from 'simple-icons';

test.describe('header', () => {
  test('title and description', async ({ page }) => {
    await page.goto('/');

    await expect(page.locator('h1')).toHaveText('Simple Icons');
    await expect(page.locator('header > :first-child > p')).toHaveText(
      `${Object.keys(icons).length} free SVG icons for popular brands`,
    );
  });

  test('nav', async ({ page }) => {
    const NAV_SELECTOR = 'header > nav';
    const N_MENU_BUTTONS = 9;

    await page.goto('/');

    if (minBreakpoint('lg', page)) {
      // desktop
      await expect(
        page.locator(`${NAV_SELECTOR} > ul:first-child`),
      ).toBeVisible();
      await expect(
        page.locator(`${NAV_SELECTOR} > ul > li:visible`),
      ).toHaveCount(N_MENU_BUTTONS);

      // burger menu is hidden
      await expect(
        page.locator(`${NAV_SELECTOR} > ul:last-child`),
      ).toBeHidden();
    } else {
      // mobile
      await expect(
        page.locator(`${NAV_SELECTOR} > ul:first-child`),
      ).toBeHidden();
      await expect(page.locator(`${NAV_SELECTOR} > ul li:visible`)).toHaveCount(
        1,
      );

      // burger menu is visible
      const burgerButton = page.locator(`${NAV_SELECTOR} > ul:last-child`);
      await expect(burgerButton).toBeVisible();
      burgerButton.click();
      await expect(
        page.locator(`${NAV_SELECTOR} > ul:first-child`),
      ).toBeVisible();
      await expect(
        page.locator(`${NAV_SELECTOR} > ul > li:visible`),
      ).toHaveCount(N_MENU_BUTTONS + 1);
    }
  });
});
