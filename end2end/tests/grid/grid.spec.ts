import { test, expect, type Page } from '@playwright/test';
import {
  N_ICONS_PER_PAGE,
  selectors,
  screenWidthIsAtLeast,
} from '../helpers.ts';

const expectNIconsLoadedInGrid = async (page: Page, nIcons: number) => {
  await page.waitForFunction(
    ({ selector, expectedNIcons }) =>
      document.querySelectorAll(selector).length === expectedNIcons,
    { selector: selectors.grid.items, expectedNIcons: nIcons },
  );
};

test.describe('grid', () => {
  test('first page of icons on load', async ({ page }) => {
    await page.goto('/');
    await expectNIconsLoadedInGrid(page, N_ICONS_PER_PAGE);
  });

  test('scroll to the footer loads the next page of icons', async ({
    page,
  }) => {
    await page.goto('/');
    await page.locator(selectors.footer).scrollIntoViewIfNeeded();
    await expectNIconsLoadedInGrid(page, N_ICONS_PER_PAGE * 2);

    await page.locator(selectors.footer).scrollIntoViewIfNeeded();
    await expectNIconsLoadedInGrid(page, N_ICONS_PER_PAGE * 3);
  });

  test('go to footer through button', async ({ page }) => {
    await page.goto('/');
    await expect(await page.locator(selectors.footer)).not.toBeInViewport();
    await page.locator(selectors.grid.scrollToFooter).click();
    await expect(await page.locator(selectors.footer)).toBeInViewport();
    await expect(await page.locator(selectors.grid.items).count()).toBe(
      N_ICONS_PER_PAGE,
    );
  });

  test('go to footer and back to header scrolling keeps loading icons', async ({
    page,
  }) => {
    await page.goto('/');
    await page.locator(selectors.grid.scrollToFooter).click();
    await expect(await page.locator(selectors.footer)).toBeInViewport();

    await page.locator(selectors.header.container).scrollIntoViewIfNeeded();
    await page.locator(selectors.footer).scrollIntoViewIfNeeded();
    await expectNIconsLoadedInGrid(page, N_ICONS_PER_PAGE * 2);
  });

  test('load more icons through button', async ({ page }) => {
    await page.goto('/');
    await page.locator(selectors.grid.scrollToFooter).click();
    await expect(await page.locator(selectors.footer)).toBeInViewport();

    await page
      .locator(selectors.grid.iconsLoader.button)
      .scrollIntoViewIfNeeded();
    await page.locator(selectors.grid.iconsLoader.button).click();
    await expectNIconsLoadedInGrid(page, N_ICONS_PER_PAGE * 2);
  });

  test('scroll to header through button', async ({ page }) => {
    await page.goto('/');
    await page.locator(selectors.grid.scrollToFooter).click();
    await expect(
      await page.locator(selectors.header.container),
    ).not.toBeInViewport();

    if (screenWidthIsAtLeast('md', page)) {
      await page.locator(selectors.grid.scrollToHeader).click();
    } else {
      // the button does not appear in mobile
      await page.locator(selectors.header.container).scrollIntoViewIfNeeded();
    }
    await expect(
      await page.locator(selectors.header.container),
    ).toBeInViewport();
  });
});
