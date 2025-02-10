# Simple Icons website client usage

### Global URL parameters

You can pass the following parameters to the website to change the behaviour on load:

| Option         | Description                                                                                                                                                                                                                        | Default                                                                                                                                                                                                         | Example               |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| `lang`         | [Unicode language identifier] of the language to use by default. It will be saved as your preferred language in local storage.                                                                                                     | The preferred language is discovered using [`navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages) based on your browser preference, using English (`en-US`) as fallback. | `?lang=es`            |
| `color-scheme` | Color scheme to use by default. Possible values are `light`, `dark` and `system`.                                                                                                                                                  | By default is system and will be inferred from your browser settings.                                                                                                                                           | `?color-scheme=light` |
| `modal`        | Show modal to open when load. Possible values are `languages` (opens the language selector), `extensions` (open the third party extensions modal) and `icon` (open the details of the first icon, combine with `q` to specify it). | By default no modals are opened.                                                                                                                                                                                | `?modal=icon&q=3m`    |
| `q`            | Value to set in search inputs. The input will depend on the page opened. It will use the value used to search along the brands at first load.                                                                                      | Depends on the visited page.                                                                                                                                                                                    | `?q=simpleicons`      |

## Index at `/`

Grid with all brand icons.

### URL parameters

|     Option      | Description                                                                                                                                                    |    Default    |        Example         |
| :-------------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-----------: | :--------------------: |
| `download-type` | The type of download to set by controls. This affect to the download button of icon cards. Possible values are `svg` and `png`.                                |     `svg`     |  `?download-type=png`  |
|    `layout`     | Icons grid layout to use. Possible values are `comfortable` and `compact`.                                                                                     | `comfortable` |   `?layout=compact`    |
|     `order`     | Order in which the icons are displayed. Possible values are `alpha` (Alphabetic), `alpha-reverse` (Alphabetic reverse), `color`, `color-reverse` and `random`. |   `random`    | `?order=color-reverse` |

[Unicode language identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier

## Preview Generator at `/preview`

Used to include an image of the icon showing a preview of it in pull requests.

### Keyboard shortcuts

- <kbd>Ctrl</kbd> + <kbd>S</kbd>: Save preview
- <kbd>Ctrl</kbd> + <kbd>C</kbd>: Copy preview
- <kbd>Ctrl</kbd> + <kbd>↓</kbd>: Download SVG
- <kbd>Ctrl</kbd> + <kbd>↑</kbd>: Upload SVG

## Deprecations at `/deprecations`

Grid with all deprecated icons. You can use the same parameters as in the index.
