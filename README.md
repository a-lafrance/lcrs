# lcrs
Sometimes I do Leetcode problems for fun; when I do, I like the extra challenge of doing them in Rust. To that end, I've collected my Rust Leetcode solutions here, just for fun. Each problem I've finished has a solution implemented in its own top-level module; all such modules are just collected into one giant library crate. In the future I'd also like to set up some way to run each solution through some kind of CLI.

Do note, though, that I tend to stay away from linked list-based problems, since those don't really play nice with Rust's ownership system. I assume graphs suffer from the same issues, but those are less common among problems I've done.
