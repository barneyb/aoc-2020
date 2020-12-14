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
REMOTE_BRANCH=`git rev-parse --abbrev-ref --symbolic-full-name @{u} | cut -d / -f 2-`
git tag -f before-start-day
git checkout master
git pull
git merge --no-ff -m "Merge branch '$BRANCH'" $BRANCH
git push
git branch -D $BRANCH
git push origin :$REMOTE_BRANCH

d=`echo $BRANCH | sed -e 's/^[^1-9]*0*\([1-9][0-9]*\).*$/\1/'`
let "d = $d + 1"
echo $d
if [ ${#d} = 1 ]; then
  d="0$d"
fi
BRANCH="day$d"
git checkout -b $BRANCH master
cat > src/the_work.rs <<EOF
use aoc_2020::read_input;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", s.len());
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "goat";

    #[test]
    fn example_one() {
        let s = EXAMPLE_ONE.trim();
        assert_eq!(4, s.len());
    }
}

EOF
