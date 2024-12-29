let SEARCHER = null;

export function build_searcher(candidates) {
  SEARCHER = new window.Searcher(candidates, {
    keySelector: (item) => item[0],
  });
}

export function search(query) {
  return SEARCHER.search(query);
}
