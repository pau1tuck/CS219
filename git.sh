#!/bin/bash

datetime=$(date "+%Y-%m-%d_%H:%M_%Z")

echo $datetime

git add .
git commit -m \"$datetime\"
git push -u origin main
