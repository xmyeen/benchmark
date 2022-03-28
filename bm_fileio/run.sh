#!/bin/sh

BATCH_BYTES="1 1024"
TOTAL_BYTES="104857600"
REPEAT_NUM=3
CMD="$@"
WORK_DIRS="$PWD"

run() {
    out_dir=$1
    for bb in $BATCH_BYTES
    do
        for tb in $TOTAL_BYTES
        do
            echo "### Work in $out_dir"
            $CMD -t $REPEAT_NUM --bb=$bb --tb=$tb --dir=$out_dir
        done
    done
}

for d in $WORK_DIRS
do
    tmpfs_dir="$d/tmpfs"
    normal_dir="$d/normal"

    # Make directory
    mkdir -p $tmpfs_dir $normal_dir

    # tmpfs
    mount -vt tmpfs -o size=1G tmpfs $tmpfs_dir
    run $tmpfs_dir
    umount -v $tmpfs_dir

    # normal
    run $normal_dir

    # Remove all
    rm -rvf $tmpfs_dir $normal_dir
done

