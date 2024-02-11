# So, how does DCMS (Doc's Customized Markdown Shorthand) work?
## It's pretty simple

All elements from markdown are unchanged, But I have added the new ^ tag (yes markdown _does_ use ^, but only sometimes
for superscript and footnotes.)

## Importing .css files

We have a folder that contains all of the css files.

To import one for use, simply do the following:
`^css colors`
This will look for `colors.css` inside of the master folder, and if it exists, it will link it.

## Using HTML macros

I cant think of a good way to automatically and dynamically call the macros by just looking up their names,
so i currently just manually make a list of all macros

To call an HTML macro, do this:
`^macro header`
this will call the `header` macro.

## Giving a page a title

just use
`^title your page title goes here`
everything after the space in `title ` will be the title, and then ends at the newline. in this case: `your page title goes here`

## Normal text

since dcms is evaluated line-by-line, doing text like the following will introduce line breaks:

```
sentence one
sentence two
sentence three
```

to do multiple lines without adding linebreaks, simply start a paragraph section with `^"`, and end it the same way.

```
^"
sentence one
sentence two
sentence three
^"
```

will become
`sentence onesentence twosentence three`
so make sure to remember your spaces!

but you can still add linebreaks manually with `^br^`

## Horizontal Rules

Same as in markdown, just do `---`

## Using custom css tags

this might change later, but for now:
`this is a bunch of ^:tagname text^ for an example.`

in this case, `tagname` is applied forwards until it hits the next ^ character in the line.
thus `text` has `tagname` applied to it.

# make sure to add new stuff as i add it pls