# ge_saves
Goldeneye Netplay Savefile Checker 

Compiled binaries are provided, however you can very easily compile it as standard rust if you wish.  It won't work unless it's in your Project 64 folder, where all the bat files are -- helpful tip: this program is a drop-in replacement for those bat files.  All those files can safely be deleted.

Error handling has only been sparsely implemented thus far, so if you run the program in the wrong folder it will panic and close (harmless, but the saves won't be checked/fixed).  Error handling to be implemented soon.

This program was written with ease of editing in mind - it should be very easy to add more options for other saves/map packs.  The handler( ) function requires two arguments - the path to the backup file, and the path to the save file.  It will then check those files against each other, and replace the savefile with the backup file if they don't match.  Add a new option to the "match" block and pass the appropriate values to the handler( ) function and it should just simply work.  If you're so inclined, you could also add a line to the check_all( ) function to add the new save to the "all" option.

If you do make changes to this program that you intend to distribute to others, please change the "version" string.  The version prints every time the program is run, and could potentially assist in troubleshooting if somebody runs into any issues (ie, they may be running a different version that doesn't have the functionality they require).  Any new versions that come into common use will replace the file in this repo.
