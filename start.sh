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

d=`echo $BRANCH | sed -e 's/^[^1-9]*0*\([1-9][0-9]*\).*$/\1/'`
let "d = $d + 1"
echo $d
if [ ${#d} = 1 ]; then
  d="0$d"
fi
BRANCH="day$d"
git checkout -b $BRANCH master
cat > src/the_work.rs <<EOF
pub fn the_work() {
    let input = aoc_2020::read_input();
    println!("{:?}", part_one(&input));
}

fn part_one(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "goat";

    #[test]
    fn example_one() {
        assert_eq!(4, part_one(&EXAMPLE_ONE));
    }
}
EOF
