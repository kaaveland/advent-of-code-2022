pbpaste | gpaste -sd+ | sed 's/++/\n/g' | bc | sort -nr | head -1