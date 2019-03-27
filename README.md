# mdbook-generate-summary

This is a CLI tool to generate the SUMMARY.md page for an [mdBook repository](https://github.com/rust-lang-nursery/mdBook).

This is a tool I wrote because I'm abusing mdBook to act as a knowledge repository, and not as an actual book.
I wanted to streamline the process of adding pages as much as possible, and not have to worry too much about maintaining that SUMMARY.md.

This does, however, make a few assumptions on the layout of your book:

Each subpage has an accompanying README.md.

E.G. Your book will always have the following directory tree under the src/:

```
foo.md
bar/README.md
bar/something_else.md
baz/README.md
```

That is, you'll never have a directory with a .md file in it, that doesn't ALSO have a README.md file.

If you do, the program won't detect that edge case, and it won't do the right thing.
