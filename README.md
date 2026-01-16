butterlog
---------

Butterlog is a CLI tool to help you visualize and scan huge log files.

It doesn't require any specific format.

Here's what it does:
- Accepts a log file as a CLI arg. 
- Figures out how big the file is
- Scans the first 5000 lines (or all, if there are less than 5000 lines), 
  and figures out the average length of each line. 
  - Based on the average length, it estimates the total number of lines
  - Then it tries to partition the file into approximately equal parts
  - Partitioning is done by grouping lines that have a similar prefix. 
    - If a partition is supposed to be large, the prefix is necessarily short
- Each partition shows up as a line in the UI, with a horizontal arrow at the 
  beginning (indicating that it's collapsed)
- The user can press down arrow key to expand a partition.
- This works recursively: a partition may internally have more partitions 
  (again based on the prefix)
- The user is also allowed to search for terms. 
  - When a search occurs, all the partitions containing the term are highlighted
  - The user can choose to navigate to a partition and expand it
