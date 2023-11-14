# Split journalctl output on boot

Quick and dirty program to split the output of journalctl into one file per boot.

## Usage
```
journalctl > filename
journal-split-on-boot filename
```

## Assumptions
* all boot are stamped with `-- Boot`
* all clean shutdown are stamped with `Journal stopped`
* If the last line is not `Journal stopped` it was a crash!

## infamous TODO (that will likely never be done!)
* derive name of unique and split files from original file
* ensure tempfile is unique (possibly all files!)
* read from stdin if no file name is supplied
* fix cause and remove `00*` from .gitignore
