#!/bin/sh

# Prompt directory to contribute
echo "Directory: "
read dirname

# Prompt commit message
echo "Commit message: "
read msg

# Access directory and delete target files
cd /home/minhdang118/rust_learn_local/${dirname}
cargo clean

# Return to workspace, add directory, commit and push to remote
cd ..
git add ${dirname}
git commit -m "${msg}"
git push