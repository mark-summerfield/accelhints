# Accelhints

Accelhints is a library for inserting keyboard Alt-key accelerators (i.e.,
'&'s) in a slice of items (e.g., menu items or dialog labels).

There is also an executable: `accelhints infile.txt > outfile.txt`.

There is also a Python 3 combined library and executable, `accelhints.py` in
the `py` subdirectory which uses the same algorithm but nonetheless produces
slightly different and slightly better results. (Diff
`tests/expected-rs.txt` with `tests/expected-py.txt` to see the
differences.)

## Example

Here the user has predefined one accelerator (Cu&t) in the input:

    Undo
    Redo
    Copy
    Cu&t
    Paste
    Find
    Find Again

Here is accelhints' output:

    &Undo
    &Redo
    &Copy
    Cu&t
    &Paste
    &Find
    Find &Again

Without help from the user the output would be:

    &Undo
    &Redo
    C&opy
    &Cut
    &Paste
    &Find
    Find &Again

This is why it is probably best to use the tool on the command line with a
text file of all the menus and dialog labels. But at a pinch the library can
be used to dynamically create acceptable if not perfect keyboard
accelerators.

## License

GPL-3.0.

## Other Free Software

[www.qtrac.eu](https://www.qtrac.eu/sitemap.html#foss).
