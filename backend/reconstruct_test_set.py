""" README
This script combines and shuffles sorted MNIST datasets.
It expects test data in a relative path of `./test_data/.
It combines and shuffles disparate datasets into one.
This script is intended for use when there are 10 lighclients involved int raining.

This script should be executed from the backend directory.
Therefore, cd into the correct directory before executing the script.

This script can be run with the following command:
`python scripts/reconstruct_test_set.py`
"""

import random
import os

print("Trying to reconstrust test data set")

files = os.listdir("./test_data/")
print(f"Files in test data directory: {files}")

all_data = []

for file in files:
    with open(f"./test_data/{file}", 'r') as f:
        data = list(f.readlines())
        first_line, data = data[0], data[1:]

    all_data.extend(data)

print(f"Length of all_data: {len(all_data)} ({len(data)} * {len(files)})")

random.shuffle(all_data)

with open("test_data.csv", 'w') as write_file:
    write_file.writelines(first_line)
    for line in all_data:
        write_file.writelines(line)

print("Successfully reconstructed test data set")