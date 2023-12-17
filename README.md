# rsortviz

A visualizer for sorting algorithms in [Rust](https://www.rust-lang.org/) using [nannou](https://nannou.cc/).

![quicksort](https://i.imgur.com/uO9Kacy.gif)

## Algorithms

- [x] [bubble sort](https://en.wikipedia.org/wiki/Bubble_sort)
- [x] [insertion sort](https://en.wikipedia.org/wiki/Insertion_sort)
- [x] [selection sort](https://en.wikipedia.org/wiki/Selection_sort)
- [x] [merge sort](https://en.wikipedia.org/wiki/Merge_sort)
- [x] [quicksort](https://en.wikipedia.org/wiki/Quicksort)
- [x] [heapsort](https://en.wikipedia.org/wiki/Heapsort)
- [x] [shellsort](https://en.wikipedia.org/wiki/Shellsort)
- [x] [radix sort (with counting sort)](https://en.wikipedia.org/wiki/Radix_sort)
- [x] [gnome sort](https://en.wikipedia.org/wiki/Gnome_sort)
- [x] [cycle sort](https://en.wikipedia.org/wiki/Cycle_sort)

## Usage

```
Options:
  -a, --algorithm <ALGORITHM>  [default: quick]
  -l, --length <LENGTH>        [default: 50]
  -h, --help                   Print help
  -V, --version                Print version
```

Possible value for `<ALGORIGTHM>` are:

- `bubble`
- `insertion`
- `selection`
- `merge`
- `quick`
- `heap`
- `shell`
- `radix`
- `gnome`
- `cycle`
