import { test, expect } from '@playwright/test';
import {
  selectors,
  useLocalStorage,
  N_ICONS,
  screenWidthIsAtLeast,
} from '../helpers.ts';

const DESCRIPTIONS = {
  en: `${N_ICONS} free SVG icons for popular brands`,
  es: `${N_ICONS} iconos SVG gratis para marcas populares`,
  fr: `${N_ICONS} icônes gratuites SVG pour les marques populaires`,
};

test.describe('language selector', () => {
  // TODO: there is way to set navigator.languages in Playwright?
  test.describe('navigator.language', () => {
    test.use({ locale: 'es-ES' });

    test('the language is auto-discovered', async ({ page }) => {
      await page.goto('/');
      await expect(page.locator(selectors.header.description)).toHaveText(
        DESCRIPTIONS.es,
      );
    });
  });

  for (const [language, description] of Object.entries(DESCRIPTIONS)) {
    test.describe(`${language} language from local storage`, () => {
      useLocalStorage(test, { language: language });

      test('autodiscover', async ({ page }) => {
        await page.goto('/');
        await expect(page.locator(selectors.header.description)).toHaveText(
          description,
        );
      });
    });
  }

  test('change language', async ({ page }) => {
    await page.goto('/');

    // English by default
    await expect(page.locator(selectors.header.description)).toHaveText(
      DESCRIPTIONS.en,
    );

    if (!screenWidthIsAtLeast('lg', page)) {
      page.locator(selectors.header.nav.toggler).click();
    }

    await page.locator(selectors.header.nav.buttons.languageSelector).click();

    const languageSelector = page.locator(selectors.modals.languageSelector);
    await expect(languageSelector).toBeInViewport();

    await languageSelector.getByText('Español').click();
    await expect(page.locator(selectors.header.description)).toHaveText(
      DESCRIPTIONS.es,
    );

    await expect(languageSelector).toBeHidden();
    await expect(
      await page.evaluate(() => localStorage.getItem('language')),
    ).toBe('es');
  });
});
