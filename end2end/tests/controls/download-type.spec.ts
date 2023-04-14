import { test, expect } from '@playwright/test';
import {
  selectors,
  minBreakpoint,
  SIMPLE_ICONS_DIRPATH,
  saveDownload,
} from '../../helpers.ts';
import * as fs from 'node:fs';
import * as path from 'node:path';

const DOWNLOAD_TYPE_CONTROL_SELECTOR =
  selectors.controls.buttons.getByNthChild(3);
const GRID_ITEM_DOWNLOAD_BUTTON_SELECTOR = `${selectors.grid.item.first} > :last-child > :last-child`;

test.describe('download type', () => {
  test('is SVG by default', async ({ page }) => {
    await page.goto('/');
    const downloadTypeButtons = await page.locator(
      `${DOWNLOAD_TYPE_CONTROL_SELECTOR} button`,
    );
    await expect(downloadTypeButtons).toHaveCount(2);
    await expect(downloadTypeButtons.nth(0)).toHaveClass('selected');
  });

  test('download SVG', async ({ page }) => {
    await page.goto('/');
    const gridItemDownloadButton = await page.locator(
      GRID_ITEM_DOWNLOAD_BUTTON_SELECTOR,
    );

    const downloadPromise = page.waitForEvent('download');
    await gridItemDownloadButton.click();
    const download = await downloadPromise;
    let filename = download.suggestedFilename();
    await expect(filename).toMatch(/[^.]+\.svg/);
    const outputPath = await saveDownload(
      download,
      `download-control-${filename}`,
    );

    const fileContent = fs.readFileSync(outputPath, 'utf8');
    const expectedFileContent = fs.readFileSync(
      path.resolve(SIMPLE_ICONS_DIRPATH, `icons/${filename}`),
      'utf8',
    );
    await expect(fileContent).toBe(expectedFileContent);
  });

  test('download PDF', async ({ page }) => {
    await page.goto('/');

    if (!minBreakpoint('lg', page)) {
      await page.locator(selectors.controls.toggler).click();
    }

    const downloadTypeButtons = await page.locator(
      `${DOWNLOAD_TYPE_CONTROL_SELECTOR} button`,
    );
    await downloadTypeButtons.nth(1).click();

    const gridItemDownloadButton = await page.locator(
      GRID_ITEM_DOWNLOAD_BUTTON_SELECTOR,
    );

    const downloadPromise = page.waitForEvent('download');
    await gridItemDownloadButton.click();
    const download = await downloadPromise;
    let filename = download.suggestedFilename();
    await expect(filename).toMatch(/[^.]+\.pdf/);
    await saveDownload(download, `download-control-${filename}`);
  });
});
