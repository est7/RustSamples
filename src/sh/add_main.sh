#!/bin/bash

# Set the target directory and file extension
target_dir="../comprehensive_rust"
file_extension="*.rs"

# Find all files with the specified extension in the target directory
for file in $(find $target_dir -type f -name "$file_extension"); do
    # Check if the file is empty
    if [ ! -s "$file" ]; then
        # Write the desired content to the empty file
        echo "#![allow(dead_code)]

pub fn main() {}" > "$file"
    fi
done