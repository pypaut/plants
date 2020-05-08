#!/bin/sh

set -x

if [ $# -ne 2 ]; then
    exit 1
fi

angle=""
dist=""
if [ $# -ge 2 ]; then
    angle="$3"
fi

if [ $# -ge 3 ]; then
    dist="$4"
fi

in_dir="$1"
out_dir="$2"
out_tmp="$2_tmp"

mkdir "$in_dir"
mkdir "$out_dir"
mkdir "$out_tmp"

echo in_dir
echo out_dir
echo out_tmp

ls_in=`ls $in_dir`
echo $ls_in

#run plants on all input files in input dir and save output to intermediary output dir
run_plants () {
    cd plants
    out_path="../$out_tmp"
    for f in $ls_in; do
        file_in="../$in_dir/$f"
        file_out="../$out_tmp/$f"
        if [[ $(file --mime-type -b "$file_in") == text/* ]]; then
            cargo run --release -- "$file_in" "$file_out" 1 2> /dev/null &
        fi
    done
    wait
    cd ..
}

run_plants
ls_tmp=`ls $out_tmp`

#run graph3D on all intermediary output and save to final output dir
run_graph () {
    cd "graph3d"
    pwd
    out_path="../$out_dir"
    for f in $ls_tmp; do
        file_in="../$out_tmp/$f"
        file_out="../$out_dir/$f"
        cargo run --release -- "$file_in" "$file_out" $angle $dist 2> /dev/null &
    done
    wait
    cd ..
}

run_graph

