
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

### Details

Each favorites json file must have overlapping id's starting
with the 2nd file that is read.  This application prints
out each file as it is read and also prints the overlapping
id's so one knows that all of the id's have been covered.

This makes creating the original json id files with
[hackernews-favorites](https://github.com/stormasm/hackernews-favorites)
relatively easy as one can error on the side of a little more redundancy
and then have this program clean that up by using the Redis set
to guarantee uniqueness and clean up the redundancy.
