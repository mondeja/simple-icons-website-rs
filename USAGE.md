# Simple Icons website client usage

## URL parameters

You can pass the following parameters to the website to change the behaviour on load:

| Option          | Description                                                                                                                                                                           | Default                                                                                                                                                                    | Example              |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| `q`             | Value to set in the search bar.                                                                                                                                                       | The search bar is empty by default and the alphabetical order mode is selected.                                                                                            | `?q=simpleicons`     |
| `lang`          | [ISO 639-1] optionally followed by an [ISO 3166-1] code separated by a character `-` of the language to use by default. It will be saved as your preferred language in local storage. | The preferred language is discovered using [`navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages) based on your browser preference. | `?lang=es`           |
| `download-type` | The type of download to set by controls. This affect to the download button of icon cards. Possible values are `svg` and `pdf`.                                                       | The download type is `svg` by default.                                                                                                                                     | `?download-type=pdf` |

[ISO 639-1]: https://en.wikipedia.org/wiki/ISO_639-1
[ISO 3166-1]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
