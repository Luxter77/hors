#!/bin/bash

runame="AlicornML"
arkdir=$(pwd)"/../pony/fimfarchive-20201201/epub"
zipdir=$(pwd)"/../pony/alzip"
copdir=$(pwd)"/../pony/corp"

mkdir $zipdir $copdir -v 2> /dev/null

set -e
# Get the zips from the epubs on the archive
echo "-------------------------------------"
for fic in $(find "$arkdir" -type f -name *.epub); do
    ffic=$(basename -- $fic | sed -e "s!.*-!!g")
    cp -u $fic "$zipdir/${ffic%.*}.zip"
done

tempfile=$copdir"/$runame-$(date +%s).txt" # temporal file
tempfolder=$(mktemp -d -t horsfilehell-XXXXX)

# Extract zips into a file hell
echo "-------------------------------------"
echo "Initializing..."
total=$(ls $zipdir | sort -V | tail -n 1 | sed -e "s!.*-!!g" -e "s!\.zip!!g")  # get the last file
for v in $(ls $zipdir | sort -V); do
	echo "-------------------------------------"
    mkdir "$tempfolder/${v%.*}"
	7z x -o"$tempfolder/${v%.*}" "$zipdir/$v" > /dev/null
	echo "[ ${v%.*} / ${total%.*} ]"
    for z in $(ls $tempfolder/${v%.*}/Chapter*.html | sort -V); do
		grep -h -v -e "<title>" -e "<h1>Author's Note</h1>" -e "Author's Note" <(echo "<|startoftext|>") $z <(echo "<|endoftext|>") | tr -s "\n" | sed  -e 's!\r!\n!g' -e 's!<p>!\n!g' -e 's!</p>!\n!g' -e 's!<br/>!\n!g' -e 's!<[^>]*>!!g' -e 's!\n!\n\n!g' -e 's/[ \t]*$//' | uniq >> $tempfile
	done
    rm -rf "$tempfolder/${v%.*}"
done
rm -rf $tempfolder
echo "-------------------------------------"
echo "Done"
