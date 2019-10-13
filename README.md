
This repo is an assistant to
[hackernews-favorites](https://github.com/stormasm/hackernews-favorites)

For now this code depends on having a set of json files
in a directory.  These json files combined add up to
a complete set of unique hacker news id's that are
the favorites on your hacker news account.

With the corresponding hackernews-favorites repo one can
create a set of different json files which MUST
contain overlapping id's so that the complete set
covers all of the id's in your favorites.

We are doing this because when one has many, many favorites
running the hackernews-favorites application takes too long
to complete.  It is much easier to run smaller runs of fewer
favorites and then combine them using this repo.

More details will follow on examples of how to run these.
