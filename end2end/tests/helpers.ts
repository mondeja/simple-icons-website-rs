import { type Page } from '@playwright/test';

type Breakpoint = 'xs' | 'sm' | 'md' | 'lg';

const getViewportSize = (page: Page): { width: number; height: number } => {
  const size = page.viewportSize();
  if (!size) {
    throw new Error('Viewport size is not set');
  }
  return size;
};

export const minBreakpoint = (br: Breakpoint, page: Page): boolean => {
  const size = getViewportSize(page);

  switch (br) {
    case 'xs':
      return size.width >= 475;
    case 'sm':
      return size.width >= 640;
    case 'md':
      return size.width >= 768;
    case 'lg':
      return size.width >= 1024;
    default:
      return false;
  }
};

export const maxBreakpoint = (br: Breakpoint, page: Page): boolean => {
  const size = getViewportSize(page);

  switch (br) {
    case 'xs':
      return size.width < 640;
    case 'sm':
      return size.width < 768;
    case 'md':
      return size.width < 1024;
    case 'lg':
      return size.width < 1280;
    default:
      return false;
  }
};
