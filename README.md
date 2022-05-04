# netflix-plan-checker

A quick and dirty script to detect overlap of users using your family plan. To do this, you request the export of your [Netflix data](https://www.netflix.com/account/getmyinfo), unzip the archive and call this script through `cargo run $PWD` with the `$PWD` being the path to your `clickstream.csv`.

This has been tested on Linux, (arch, btw). It will detect users watching withing 1 hour of each other, as we have no way to tell how long their actual playback is. In order to do this, the script will filter for a "Navigation" value of either "playback", "postPlay" or "progressSpinner". It also reduces the result down to days with overlap.

The result will look like this:

```
(2022-04-07, {"User1", "User2"})
(2022-04-08, {"User2", "User1", "User3"})
(2022-04-11, {"User1", "User3"})
``` 
where the usernames are instead populated by the exact profile names.
---
Due to an implementation quirk, you might also run into late night playbacks that overlap with either the next or previous day, in that case they might occassionally be put into a date of their own, like this:
```
(2022-04-12, {"User3"})
```
