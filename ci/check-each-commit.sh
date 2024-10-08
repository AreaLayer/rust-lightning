#!/bin/sh
if [ "$1" = "" ]; then
	echo "USAGE: $0 remote/head_branch"
	echo "eg $0 upstream/main"
	exit 1
fi

set -e
set -x

if [ "$(git log --pretty="%H %D" | grep "^[0-9a-f]*.* $1")" = "" ]; then
	echo "It seems like the current checked-out commit is not based on $1"
	exit 1
fi
git rebase --exec ci/check-compiles.sh "$1"
