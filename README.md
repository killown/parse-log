# Parse LOG
## _rust tail implementation to parse files_

## install
https://pypi.org/project/parse-log/https://pypi.org/project/parse-log/

## Features

- Load N lines or all lines
- search and ignore lines with certain strings
- parse lines and extract what you want

## Usage
```python
import tail

#search for lines with certain strings and also ignore lines with certain strings
#search and ignore use spaces as delimitator
number_of_lines = 10
tail.search_lines(path_to_file, "search1, search2...", "ignore_string1, ignore_string2...", number_of_lines)
tail.search_lines(path_to_file, "search", "", number_of_lines)


#return 3 lines
number_of_lines = 3
tail.lines(path_to_file, number_of_lines)

#return all lines
number_of_lines = 0
tail.lines(path_to_file, number_of_lines)

#parse line by line
number_of_lines = 3
lines = tail.lines(path_to_file, number_of_lines)
[tail.parse_line(lines, "begin_delimiter", "end_delimiter") for line in lines]
```
