#!/bin/sh
# Recommended command-line:
#
# commit-db.rb list-valid nightly|GIT_DIR=/your/rust/dir/.git sync.sh

cd "$(dirname "$0")"
for COMPILER_COMMIT in $(sort -u); do
	COLL_COMMIT=$(git log -n1 --pretty=format:%H $COMPILER_COMMIT -- src/libstd/collections)
	if ! [ -d src/$COLL_COMMIT ]; then
		mkdir src/$COLL_COMMIT
		git archive $COLL_COMMIT src/libstd/collections|tar xf - -C src/$COLL_COMMIT --strip-components=3
	fi
	if ! grep -q $COMPILER_COMMIT mapping.rs; then
		echo "-Mapping(\"$COMPILER_COMMIT\",\"$COLL_COMMIT\")" >> mapping.rs
	fi
done
