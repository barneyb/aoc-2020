#!/usr/bin/env bash
set -e

cd `dirname $0`

if [ `git status --porcelain | wc -l` != "0" ]; then
  echo "Your working copy is dirty!"
  exit 1
fi

BRANCH=`git rev-parse --abbrev-ref HEAD`
if [ "$BRANCH" = "master" ]; then
  echo "You're already on master. You probably want:"
  echo
  echo "    git checkout -b dayXX"
  echo
  exit 0
fi
git push --set-upstream origin $BRANCH
git tag -f before-start-day
git checkout master
git pull
git merge --no-ff -m "Merge branch '$BRANCH'" $BRANCH
git push
git branch -D $BRANCH

y=`echo $BRANCH | sed -e 's/^[^1-9]*0*\([1-9][0-9]*\).*$/\1/'`
let "d = $y + 1"
echo $d
if [ ${#d} = 1 ]; then
  d="0$d"
fi
git checkout -b day$d master
mkdir src/day$d
sed -i -e "s/aoc_2020::day${y}_.*::solve/aoc_2020::day$d::solve/" src/main.rs
sed -i -e "s/\(pub mod day${y}_.*;\)/\\1\\npub mod day$d;/" src/lib.rs
cp template/*.rs src/day$d
git add src/main.rs src/lib.rs src/day$d
