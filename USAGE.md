# Simple Icons website client usage

## URL parameters

You can pass the following parameters to the website to change the behaviour on load:

| Option | Description                                                                                                                                                | Default                                                                                                                                   | Example          |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| `q`    | Value to set in the search bar.                                                                                                                            | The search bar is empty by default and the alphabetical order mode is selected.                                                           | `?q=simpleicons` |
| `lang` | [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) code of the language to use by default. It will be saved as your preferred language in local storage. | The preferred language is discovered using [`navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages). | `?lang=es`       |
