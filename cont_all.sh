#!/bin/sh

# Prompt commit message
printf "Commit message: "
read msg

# Add everything, commit and push to remote
git add .
git commit -m "${msg}"
git push
