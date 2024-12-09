# Pairwise edit distance between pangenome graphs


Program that calculates the distance between two GFA (Graphical Fragment Assembly) files. It takes in the file paths of the two GFA files. The program first identifies the common paths between the two graphs by finding the intersection of their path names. For each common path, the program reads those and output differences in segmentation in-between them. The purpose is to output the necessary operations (merges and splits) required to transform the graph represented by the first GFA file into the graph represented by the second GFA file.

![edition algorithm](https://github.com/dubssieg/dubssieg/blob/main/algorithm.gif)

## Install instructions:

Find the latest pre-compiled binaries [in the release page here](https://github.com/dubssieg/rs-pancat-compare/releases).

Build from source: requires rust and cargo.

```bash
git clone 
cd rs-pancat-compare
cargo build --release
```

## Usage

```bash
./target/release/rs-pancat-compare graph_A.gfa graph_B.gfa > output.tsv
```

:warning: Only accept P-lines as paths in GFA files - please convert your GFA1.1 files to GFA1.0 with vg for instance. 


| Organism | Chromosom | Wall time | Memory |
|----------|-----------|-----------|--------|
| yeast    | 1         | 0.56s     | 3.0MB  |
| human    | 21        | 5m08s     | 383MB  |
| human    | 1         | 17m42s    | 1.2GB  |

Timings and peak memory usage over diverse datasets. Jobs executed on a single core of a 13th Gen Intel® Core™ i7-1365U @ 3.6GHz.

> [!NOTE]\
> Want to contribute? Feel free to open a PR on an issue about a missing, buggy or incomplete feature! **Please do bug reports in the issue tracker!**.

## Output

Program outputs to `stdout` in a `.tsv` format editions as well as informations about the comparison.

```
# Intersection of paths: [pathname:str,+]
## pathname:str	pathlength:int
# Path name	Position	Operation	NodeA	NodeB	BreakpointA	BreakpointB
pathname:str	[0-9]+:int	[M|S]:str	[0-9]+:str	[0-9]+:str	[0-9]+:int	[0-9]+:int
...
# Distance: [0-9]+:int (E=[0-9]+:int, S=[0-9]+:int, M=[0-9]+:int).

```

Output features:
+ Lines starting with '#' are comments or information about the comparison
+ Lines starting with '##' are haplotypes length information
+ Every other line is either a merge (M) or a split (S)
+ Equivalences are accounted in the final line but not written as output (too many in file)
+ Distance is the sum of merges and splits
+ `Path name` is the haplotype name string
+ `Position` is the global position on the graph the edit takes place
+ `NodeA` (resp. `NodeB`) is the node on pathA (resp. pathB) where the edition occurs
+ `BreakpointA` (resp. `BreakpointB`) is the next breakpoint position on pathA (resp. pathB)

## Test datasets

You can find datasets used for the paper [on Zenodo](https://zenodo.org/records/10932490) and instructions on how to use [on the dedicated repository](https://github.com/dubssieg/pancat_paper).

> [!NOTE]\
>  [pancat compare](https://github.com/Tharos-ux/pancat) is a tool, originally written in Python, designed to compute a distance between pangenome graphs made from a same group of genomes. For performance, it has been reimplemented in Rust.
