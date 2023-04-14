import { test, expect, type Page } from '@playwright/test';
import { screenWidthIsAtLeast, selectors } from '../helpers.ts';

const LAYOUT_CONTROL_SELECTOR =
  'menu > .controls-group:not(:first-child) > .control:last-child';

test.describe('layout', () => {
  test('is comfortable by default', async ({ page }) => {
    await page.goto('/');
    const layoutButtons = await page.locator(
      `${LAYOUT_CONTROL_SELECTOR} button`,
    );
    await expect(layoutButtons).toHaveCount(2);
    await expect(layoutButtons.nth(0)).toHaveClass('selected');
  });

  test.describe('comfortable -> compact', () => {
    test('switch', async ({ page }) => {
      await page.goto('/');
      const gridItem = await page.locator(selectors.grid.item.any.container);
      const { height: prevHeight, width: prevWidth } =
        await gridItem.boundingBox();

      if (!screenWidthIsAtLeast('lg', page)) {
        await page.locator(selectors.controls.toggler).click();
      }

      const layoutButtons = await page.locator(
        `${LAYOUT_CONTROL_SELECTOR} button`,
      );
      await layoutButtons.nth(1).click();
      await expect(layoutButtons.nth(1)).toHaveClass('selected');

      const { height: newHeight, width: newWidth } =
        await gridItem.boundingBox();
      expect(newHeight).toBeLessThan(prevHeight);
      expect(newWidth).toBeLessThan(prevWidth);

      // check that the layout is saved in localStorage
      await page.reload();
      await expect(
        await page.locator(`${LAYOUT_CONTROL_SELECTOR} button`).nth(1),
      ).toHaveClass('selected');
    });
  });

  test.describe('compact -> comfortable', () => {
    test.describe.configure({ mode: 'serial' });

    let page: Page;

    test.beforeAll(async ({ browser }) => {
      page = await browser.newPage();
      await page.goto('/');
      await page.evaluate(() => {
        localStorage.setItem('layout', 'compact');
      });
    });

    test.afterAll(async () => {
      await page.close();
    });

    test('switch', async () => {
      page.reload();

      const gridItem = await page.locator(selectors.grid.item.any.container);
      const { height: prevHeight, width: prevWidth } =
        await gridItem.boundingBox();

      if (!screenWidthIsAtLeast('lg', page)) {
        await page.locator(selectors.controls.toggler).click();
      }

      const layoutButtons = await page.locator(
        `${LAYOUT_CONTROL_SELECTOR} button`,
      );
      // Compact has been loaded from localStorage
      await expect(layoutButtons.nth(1)).toHaveClass('selected');
      await layoutButtons.nth(0).click();
      await expect(layoutButtons.nth(0)).toHaveClass('selected');

      const { height: newHeight, width: newWidth } =
        await gridItem.boundingBox();
      expect(newHeight).toBeGreaterThan(prevHeight);
      expect(newWidth).toBeGreaterThan(prevWidth);
    });
  });
});
