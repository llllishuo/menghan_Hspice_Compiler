#!/bin/bash
echo $path
for i in $(ls src/data/*.sp); do
	echo "当前执行为: $i"
	iter=$(cargo run -- $i)
	for item in $iter; do
		if [[ $item == "<WARN>"* ]]; then
			echo $item
			
		fi
	done
done

