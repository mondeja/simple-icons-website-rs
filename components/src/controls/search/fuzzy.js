let SEARCHER = null;

export function rebuild_searcher(candidates) {
  console.log(candidates[0]);
  candidates.forEach((candidate) => {
    console.log(candidate);
  });
  SEARCHER = new window.Searcher(candidates, {
    keySelector: (item) => item[0],
  });
}

export function search(query) {
  return SEARCHER.search(query);
}
