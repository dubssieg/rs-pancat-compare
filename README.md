# Pairwise edit distance between pangenome graphs


Program that calculates the distance between two GFA (Graphical Fragment Assembly) files. It takes in the file paths of the two GFA files. The program first identifies the common paths between the two graphs by finding the intersection of their path names. For each common path, the program reads those and output differences in segmentation in-between them. The purpose is to output the necessary operations (merges and splits) required to transform the graph represented by the first GFA file into the graph represented by the second GFA file.

![edition algorithm](https://github.com/dubssieg/dubssieg/blob/main/algorithm.gif)

## Install instructions:

(For Linux-based systems only) Find the latest pre-compiled binaries [in the release page here](https://github.com/dubssieg/rs-pancat-compare/releases).

(For anyone) Build from source: requires rust and cargo.

```bash
git clone 
cd rs-pancat-compare
cargo build --release
```

## Usage

:warning: Only accept P-lines as paths in GFA files - please convert your GFA1.1 files to GFA1.0 with the vg toolkit for instance. 

```bash
./target/release/rs-pancat-compare example/graph_A.gfa example/graph_B.gfa > output.tsv
```

On included graphs (in `example/` folder), you should obtain this output:

```bash
# Intersection of paths: ["CASBJH01", ... "CASBJS01"]
## CASBJH01     219308
...
## CASBJS01     206475
# Path name     Position        Operation       NodeA   NodeB   BreakpointA     BreakpointB
CASBJH01        20      S       15707   21230   20      7565
CASBJH01        21      S       15706   21230   21      7565
CASBJH01        23      S       15704   21230   23      7565
...
CASBJU01        222414  M       21721   23661   222416  222414
CASBJU01        222416  S       21721   23662   222416  222417
CASBJU01        222418  S       21723   23663   222418  222419
# Distance: 34203 (E=208247, S=21435, M=12768).
```

The order of the graphs is used to qualify editions. It is computed as "the minimal set of required operations to obtain the graph B out of the graph A".


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


## Citing

 Pairwise graph edit distance characterizes the impact of the construction method on pangenome graphs
Siegfried Dubois, Claire Lemaitre, Matthias Zytnicki, Thomas Faraut
bioRxiv 2024.12.06.627166; doi: [https://doi.org/10.1101/2024.12.06.627166](https://doi.org/10.1101/2024.12.06.627166)

### Bibtex:

```
@article {dubois_distance_2024,
	author = {Dubois, Siegfried and Lemaitre, Claire and Zytnicki, Matthias and Faraut, Thomas},
	title = {Pairwise graph edit distance characterizes the impact of the construction method on pangenome graphs},
	elocation-id = {2024.12.06.627166},
	year = {2024},
	doi = {10.1101/2024.12.06.627166},
	publisher = {Cold Spring Harbor Laboratory},
	abstract = {Pangenome variation graphs are an increasingly used tool to perform genome analysis, aiming to replace a linear reference in a wide variety of genomic analyses. The construction of a variation graph from a collection of chromosome-size genome sequences is a difficult task that is generally addressed using a number of heuristics. The question that arises is to what extent the construction method influences the resulting graph, and the characterization of variability. We aim to characterize the differences between variation graphs derived from the same set of genomes with a metric which expresses and pinpoint differences. We designed a pairwise variation graph comparison algorithm, which establishes an edit distance between variation graphs, threading the genomes through both graphs. We applied our method to pangenome graphs built from yeast and human chromosome collections, and demonstrate that our method effectively characterizes discordances between pangenome graph construction methods and scales to real datasets.Competing Interest StatementThe authors have declared no competing interest.},
	URL = {https://www.biorxiv.org/content/early/2024/12/10/2024.12.06.627166},
	eprint = {https://www.biorxiv.org/content/early/2024/12/10/2024.12.06.627166.full.pdf},
	journal = {bioRxiv}
}
```