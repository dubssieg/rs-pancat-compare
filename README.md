# rs-pancat-compare

> [!NOTE]\
>  [pancat compare](https://github.com/Tharos-ux/pancat) is a tool, originally written in Python, designed to compute a distance between pangenome graphs made from a same group of genomes. For performance, it has been reimplemented in Rust.

Program that calculates the distance between two GFA (Graphical Fragment Assembly) files. It takes in the file paths of the two GFA files. The program first identifies the common paths between the two graphs by finding the intersection of their path names. For each common path, the program reads those and output differences in segmentation in-between them. The purpose is to output the necessary operations (merges and splits) required to transform the graph represented by the first GFA file into the graph represented by the second GFA file.

![edition algorithm](https://github.com/dubssieg/dubssieg/blob/main/algorithm.gif)

> [!NOTE]\
> Want to contribute? Feel free to open a PR on an issue about a missing, buggy or incomplete feature! **Please do bug reports in the issue tracker!**.