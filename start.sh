#!/usr/bin/env bash
set -e

cd `dirname $0`

if [ `git status --porcelain | wc -l` != "0" ]; then
  echo "Your working copy is dirty!"
  exit 1
fi

git tag -f before-start-day
BRANCH=`git rev-parse --abbrev-ref HEAD`
REMOTE_BRANCH=`git rev-parse --abbrev-ref --symbolic-full-name @{u} | cut -d / -f 2-`
git checkout master
git pull
git merge --no-ff -m "Merge branch '$BRANCH'" $BRANCH
git push
git branch -D $BRANCH
git push origin :$REMOTE_BRANCH

let "d = ${BRANCH//[^0-9]} + 1"
echo $d
if [ ${#d} = 1 ]; then
  d="0$d"
fi
BRANCH="day$d"
git checkout -b $BRANCH master
