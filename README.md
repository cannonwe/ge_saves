# ge_saves
Goldeneye Netplay Savefile Checker 

Compiled binaries for Windows will be uploaded at a later time.  In the meantime, you can very easily compile it as standard rust.  It won't work unless it's in your Project 64 folder, where all the  bat files are.  

Error handling has only been sparsely implemented thus far, so if you run the program in the wrong folder it will panic and close (harmless, but the saves won't be checked/fixed).  Error handling to be implemented soon.

This program was written with ease of editing in mind - it should be very easy to add more options for other saves/map packs.  The handler() function requires two arguments - the path to the backup file, and the path to the save file.  It will then check those files against each other, and replace the savefile with the backup file if they don't match.  Add a new option to the "match" block and pass the appropriate values to the handler() function and it should just simply work.  If you're so inclined, you could also add a line to the check_all() function to add the new save to the "all" option.

If you do make changes to this program that you intend to distribute to others, please change the "version" string.  The version prints every time the program is run, and could potentially assist in troubleshooting if somebody runs into any issues (ie, they may be running a different version that doesn't have the functionality they require).  Any new versions that come into common use will replace the file in this repo.

Consider compiling a Windows binary for me!  I can upload it here!  This will save me a considerable headache, as I am primarily using Linux.
