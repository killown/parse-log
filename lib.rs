/*Copyright (c) 2018 konstin

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.*/



use pyo3::prelude::*;
use std::fs::{File};
use std::io::{Read, Seek, SeekFrom};
use std::io::{BufRead, BufReader};

pub const BLOCK_SIZE: u64 = 1 << 16;
pub struct ReverseChunks<'a> {
    file: &'a File,
    size: u64,
    max_blocks_to_read: usize,
    block_idx: usize,
}

impl<'a> ReverseChunks<'a> {
    pub fn new(file: &'a mut File) -> ReverseChunks<'a> {
        let size = file.seek(SeekFrom::End(0)).unwrap();
        let max_blocks_to_read = (size as f64 / BLOCK_SIZE as f64).ceil() as usize;
        let block_idx = 0;
        ReverseChunks {
            file,
            size,
            max_blocks_to_read,
            block_idx,
        }
    }
}

impl<'a> Iterator for ReverseChunks<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.block_idx >= self.max_blocks_to_read {
            return None;
        }
        let block_size = if self.block_idx == self.max_blocks_to_read - 1 {
            self.size % BLOCK_SIZE
        } else {
            BLOCK_SIZE
        };
        let mut buf = vec![0; BLOCK_SIZE as usize];
        let pos = self
            .file
            .seek(SeekFrom::Current(-(block_size as i64)))
            .unwrap();
        self.file
            .read_exact(&mut buf[0..(block_size as usize)])
            .unwrap();
        let pos2 = self
            .file
            .seek(SeekFrom::Current(-(block_size as i64)))
            .unwrap();
        assert_eq!(pos, pos2);

        self.block_idx += 1;

        Some(buf[0..(block_size as usize)].to_vec())
    }
}

fn backward(file: &mut File, num_delimiters: u64, delimiter: u8) {
    let mut counter = 0;
    for (block_idx, slice) in ReverseChunks::new(file).enumerate() {
        let mut iter = slice.iter().enumerate().rev();
        if block_idx == 0 {
            if let Some(c) = slice.last() {
                if *c == delimiter {
                    iter.next();
                }
            }
        }
        for (i, ch) in iter {
            if *ch == delimiter {
                counter += 1;
                if counter >= num_delimiters {
                    file.seek(SeekFrom::Current((i + 1) as i64)).unwrap();
                    return;
                }
            }
        }
    }
}

// split lines based on the given delimiters
fn extract_from_line(line: &str, begin_delimiter: &str, end_delimiter: &str)  -> String {
    let mut value = line;
    if begin_delimiter.is_empty() == true && end_delimiter.is_empty() == true {
     value = line;
    }
    if begin_delimiter.is_empty() == false && end_delimiter.is_empty() == false {
        value = line
       .split(begin_delimiter)
       .nth(1)
       .unwrap()
       .split(end_delimiter)
       .next()
       .unwrap();
       }
    return value.to_string();
}

#[pyfunction]
// get the line in list format
fn parse_line(line: &str, begin_delimiter: &str, end_delimiter: &str)-> PyResult<String> {
    let r = extract_from_line(line, begin_delimiter, end_delimiter);
    Ok(r)
}

// return the total number of file lines
pub fn count_lines(path: &str) -> u64 {
    let file = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut cnt  = 0;

    for _ in file.lines() {
        cnt = cnt + 1;
    }

    return cnt;
}

// drop line if it does not contains the given &str
fn match_lines(matchers: &[&str], line: &str) -> bool {
    for m in matchers {
        if line.contains(m){
            return true;
        }
    }
    return false;
}

// search and || or ignore lines based on the given strings
fn extract_lines(lines: String, begin_delimiter: &str, end_delimiter: &str, search: &str, ignore: &str) -> Vec<String> {
    let mut parsed_lines = Vec::new();
    // search with one or mores args "search1, search2" converted into list
    let search_lines:  Vec<&str> = search
                                  .split(",")
                                  .map(|s| s.trim())
                                  .filter(|s| !s.is_empty())
                                  .collect::<Vec<_>>();
    let ignore_lines: Vec<&str> = ignore
                                  .split(",")
                                  .map(|s| s.trim())
                                  .filter(|s| !s.is_empty())
                                  .collect::<Vec<_>>();
    // search, ignore or both then return parsed lines
    for line in lines.split("\n") {
        if ignore != "" {
            if match_lines(&ignore_lines, line) {
                continue;
            }
        }
        // chars in search_lines splited by must match with the actual line
        if match_lines(&search_lines, line) {
            let parsed_line = extract_from_line(line, begin_delimiter, end_delimiter);
            // after parse the line we trim it case it's necessary, before push into the parsed lines
            if !parsed_line.is_empty() {
                let result_line;
                let trim_line = parsed_line.trim();
                if trim_line.len() < parsed_line.len() {
                    result_line = trim_line.to_string();
                }
                else{
                    result_line = parsed_line;
                }
                parsed_lines.push(result_line);
            }
        }
    }
    return parsed_lines;
}


fn fsearch(filename: &str, search: &str, ignore: &str, ignore_mode: bool, number_of_lines: u64) -> Vec<String> {
    let mut n = number_of_lines;
    //if argument for number_of_lines is 0, then use the entire file
    if n == 0 {
        n = count_lines(filename);
    }
    let r;
    let mut contents =  File::open(&filename)
                    .expect("Something went wrong reading the file");
    if ignore_mode {
        r = tail_parse(&mut contents, n, "", "", search, ignore);
    }
    else{
        r = tail_parse(&mut contents, n, "", "", search, "");
    }
    return r;
}

fn fsearch_last_line(filename: &str, search: &str, ignore: &str, ignore_mode: bool) -> Vec<String> {
    let mut counter = 0u64;
    let mut search_found;
    loop {
        counter += 1;
        if ignore_mode {
            search_found = fsearch(filename, search, ignore, true, counter);
        }
        else{
            search_found = fsearch(filename, search, "", false, counter);
        }
        if !search_found.is_empty() {
            break;
        }
    }
    return search_found;
}

#[pyfunction]
fn search_last_line(filename: &str, search: &str) -> PyResult<Vec<String>> {
    let r = fsearch_last_line(filename, search, "", false);
    Ok(r)
}

#[pyfunction]
fn isearch_last_line(filename: &str, search: &str, ignore: &str) -> PyResult<Vec<String>> {
    let r = fsearch_last_line(filename, search, ignore, true);
    Ok(r)
}

#[pyfunction]
fn search_lines(filename: &str, search: &str, number_of_lines: u64) -> PyResult<Vec<String>> {
    let r = fsearch(filename, search, "", false, number_of_lines);
    Ok(r)
}

#[pyfunction]
fn search_all(filename: &str, search: &str) -> PyResult<Vec<String>> {
    let r = fsearch(filename, search, "", false, 0);
    Ok(r)
}

#[pyfunction]
fn isearch_all(filename: &str, search: &str, ignore: &str) -> PyResult<Vec<String>> {
    let ignore_mode = true;
    let r = fsearch(filename, search, ignore, ignore_mode, 0);
    Ok(r)
}

#[pyfunction]
fn isearch_lines(filename: &str, search: &str, ignore: &str, number_of_lines: u64) -> PyResult<Vec<String>> {
    let ignore_mode = true;
    let r = fsearch(filename, search, ignore, ignore_mode, number_of_lines);
    Ok(r)
}

fn tail_parse(file: &mut File, lines_number: u64, begin_delimiter: &str, end_delimiter: &str, search: &str, ignore: &str) -> Vec<String> {
    // Find the position in the file to start printing from.
    let delimiter = b'\n';
    backward(file, lines_number, delimiter);
    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("The string cannot be read");
    let vec = extract_lines(lines, begin_delimiter, end_delimiter, search, ignore);
    return vec;
}

#[pyfunction]
 fn lines(filename: &str, number_of_lines: u64)  -> PyResult<Vec<String>> {
    let mut n = number_of_lines;
    //if argument for number_of_lines is 0, then use the entire file
    if n == 0 {
        n = count_lines(filename);
    }
    let mut contents =  File::open(&filename)
                    .expect("Something went wrong reading the file");
    let r = tail_parse(&mut contents, n, "", "", "", "");
    Ok(r)
}

#[pymodule]
fn tail(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lines, m)?)?;
    m.add_function(wrap_pyfunction!(parse_line, m)?)?;
    m.add_function(wrap_pyfunction!(search_lines, m)?)?;
    m.add_function(wrap_pyfunction!(isearch_lines, m)?)?;
    m.add_function(wrap_pyfunction!(search_all, m)?)?;
    m.add_function(wrap_pyfunction!(isearch_all, m)?)?;
    m.add_function(wrap_pyfunction!(search_last_line, m)?)?;
    m.add_function(wrap_pyfunction!(isearch_last_line, m)?)?;
    Ok(())
}
