/**
 * Tests for header component.
 **/

import { test, expect } from '@playwright/test';
import { minBreakpoint, maxBreakpoint } from './helpers.ts';

import * as icons from 'simple-icons';

test.describe('header', () => {
  test('title and description', async ({ page }) => {
    await page.goto('/');

    await expect(page.locator('h1')).toHaveText('Simple Icons');
    await expect(page.locator('header > :first-child > p')).toHaveText(
      `${Object.keys(icons).length} free SVG icons for popular brands`,
    );
  });

  test.describe('nav', () => {
    const NAV = 'header > nav';
    const N_MENU_BUTTONS = 9;

    test('nav buttons are visible on desktop', async ({ page }) => {
      test.skip(!minBreakpoint('lg', page));

      await page.goto('/');
      await expect(page.locator(`${NAV} > ul:first-child`)).toBeVisible();
      await expect(page.locator(`${NAV} > ul > li:visible`)).toHaveCount(
        N_MENU_BUTTONS,
      );

      // burger menu is hidden
      await expect(page.locator(`${NAV} > ul:last-child`)).toBeHidden();
    });

    test('use nav burger menu on mobile', async ({ page }) => {
      test.skip(!maxBreakpoint('md', page));

      await page.goto('/');
      await expect(page.locator(`${NAV} > ul:first-child`)).toBeHidden();
      await expect(page.locator(`${NAV} > ul li:visible`)).toHaveCount(1);

      // burger menu is visible
      const burgerButton = page.locator(`${NAV} > ul:last-child`);
      await expect(burgerButton).toBeVisible();
      burgerButton.click();
      await expect(page.locator(`${NAV} > ul:first-child`)).toBeVisible();
      await expect(page.locator(`${NAV} > ul > li:visible`)).toHaveCount(
        N_MENU_BUTTONS + 1,
      );
    });
  });
});
