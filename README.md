# Zipf
Zip archiver which removes git artifacts and uses recursion by default.

## Usage
zipf out.zip path/to/content -x path/to/excluded/artifacts

for example for a rust project you would likely want to exclude the `target directory`

zipf out.zip . -x target

`.git` and `.gitignore` are always ignored.
