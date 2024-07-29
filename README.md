## Slides

This was my first real project in Rust. The inspiration was from downloading lecture slides from professors that were constantly named something useless like "lecture1".
The program takes an [Operator](#operations), [Format](#format) and optional [Directory](#directory) argument. It was designed for Windows systems, and to be run through
PowerShell by invoking either cargo run or the executable directly, henceforth referred to as `slides.exe`. Limited testing has been done on Git Bash or Linux terminals. Also be sure to see the [Caution](#a-word-of-caution) at the bottom.

### Operations

The first argument taken is the operation. There are 4 supported operations: [Prefix](#prefix), [Suffix](#suffix), [Version Remove](#version-remove), and [Pattern](#pattern).

### Format

The second argument taken is the format. The format required is different for each operation, but they all follow a general pattern. For 
[Prefix](#prefix), [Suffix](#suffix), and [Version Remove](#version-remove) the `*` represents where the current file name will be placed, while for [Pattern](#pattern) 
it represents where the number should be placed. Just `*` is valid for both Version Remove and Pattern, but if you are getting an error, try escaping it, like `\*`. In all formats, the file extension is optional. If it is included, then only files with that extension will be modified,
and if there is no extension then all files in the directory will be modified. 

Note: in the examples below if the file extension is omitted I use `.all` to show ALL files in the directory will be modified.

### Directory

Finally, the user can pass in a directory to operate on. If no directory is supplied, the directory where the executable is contained is used.

---
Now, for a little more detail on each specfic case.

### Prefix

Command: pre, prefix

Description: The Prefix simply appends the pattern passed in to the front of the files

Format: The text to be prefixed, followed by `*`, with the optional file extension

Examples: 
```
~$ slides.exe pre hello_*.txt \~
world.txt -> hello_world.txt

~$ slides.exe prefix 400*
test.all -> 400test.all
```

### Suffix

Command: suf, suffix

Description: The Suffix is also simple, just appending the pattern passed in to the end of the file names

Format: Starts with `*`, followed by the text to be suffixed, and the optional file extension

Examples: 
```
~$ slides.exe suf *_bar ..\..\temp\
foo.all -> foo_bar.all

~$ slides.exe suffix *_ferris.rs
i_love.rs -> i_love_ferris.rs
```

### Version Remove

Command: vr

Description: On Windows, downloading files with the same name will add a version tag onto it, something like the (1) in `download (1).txt`. This operation removes all version
tags from the files. Notice, if two files would have the same name after the tag is removed, then the rename won't take place.

Format: Either just `*` to remove versions from all files, or `*` followed by a file extension to remove versions from those files

Examples: 
```
~$ slides.exe vr *
foo (1).all -> foo.all
bar (12).all -> bar.all

~$ slides.exe vr *.pdf \testing\
number5 (2).pdf -> number5.pdf
```

### Pattern

Command: pat, pattern

Description: This is designed to rename files with numbers that mark order, providing a new name for all files while keeping the order. Some things to keep in mind is that
it will only update 1 or 2 digit numbers. Additionally, the number taken from the previous file is the **leftmost** 1 or 2 digit number. This is in an attempt to avoid class
numbers, dates, version tags, or any other numbers from interfering. Just to reiterate, the digits taken to keep order are the leftmost digits of length 1 or 2

Format: Optional text, a `*` operator, and more optional text, with the optional file extension. The `*` will be replaced with a number.

Examples: 
```
~$ slides.exe pat This_is_lecture_*.pptx
lec1.pptx -> This_is_lecture_1.pptx
lec2.pptx -> This_is_lecture_2.pptx
...
lec100.pptx -> lec100.pptx

~$ slides.exe pattern *_spring2022
fall2021_1.all -> 1_spring2022.all
fall2021_2.all -> 2_spring2022.all
...
```
---

### A Word of Caution

Be careful!! I have spent hours testing this code, but it is still code! While testing this I accidentally renamed files I did not want to, 
and the old names are unrecoverable. It is entirely possible that an overlooked bug could cause 
irreversable file loss. I recommend putting files that you want to be renamed in their own directory, and making backups. You have been warned!
