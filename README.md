# Parse LOG
## _rust tail implementation to parse files_

## Features

- Load N lines or all lines
- search and ignore lines with certain strings
- parse lines and extract what you want

## Usage
```python
import tail

path_to_file = 'path/to/file/thefile.log'
number_of_lines = 3

#return 3 lines
tail.lines(path_to_file, number_of_lines)

#return all lines with the given search string
#or 'search1, search2'
tail.search_all(path_to_file, 'Search_String')

#return all lines with the given search string
#or 'search1, search2' also 'ignore1, ignore2' and so on
tail.isearch_all(path_to_file, 'Search_String', 'Ignore_String')

#search through all lines contains certain string condition
#or 'search1, search2' also 'ignore1, ignore2' and so on
tail.search_lines(path_to_file, "search", "ignore_string", number_of_lines)

#search through all lines and ignore lines that contains certain stuff
#or 'search1, search2' also 'ignore1, ignore2' and so on
tail.isearch_lines(path_to_file, 'Search_String', 'Ignore_string', number_of_lines)

#return the last line based on the search condition
#or 'search1, search2'
tail.search_last_line(path_to_file, 'Search_String')

#return last line based on search and ignore conditions
#or 'search1, search2' also 'ignore1, ignore2' and so on
tail.isearch_last_line(path_to_file, 'Search_String', 'Ignore_String')

#parse line by line
lines = tail.lines(path_to_file, number_of_lines)
[tail.parse_line(lines, "begin_delimiter", "end_delimiter") for line in lines]
```
