# lcrs
Sometimes I do Leetcode problems for fun; when I do, I like the extra challenge of doing them in Rust. To that end, I've collected my Rust Leetcode solutions here, just for fun. Each problem I've finished has a solution implemented in its own top-level module; all such modules are just collected into one giant library crate. In the future I'd also like to set up some way to run each solution through some kind of CLI.

Some problems have multiple solutions, depending on the time and space trade-offs associated with them. I'll usually only include alternate solutions that are especially interesting and make a worthwhile trade-off (eg most of the time brute force solutions save lots of memory but aren't particularly interesting or tenable in practice), and will tend to favor speed over memory usage in general.

Do note, though, that I tend to stay away from linked list-based problems, since those don't really play nice with Rust's ownership system. I assume graphs suffer from the same issues, but those are less common among problems I've done.
