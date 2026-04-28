# View edit distance

You may want to see on local, known parts of your genomes how the edits are spreaded. This small script gives you a graphical representation for the distance.

Please note that this is only proof of concept and should be used only on subgraphs (1kb average maximum - depending on your graph).

Example: view between positions 0 and 200 according to path CASBJU01 and output result in `output.html`

```bash
python visualisation.py mc_yeast_1.gfa pggb_yeast_1.gfa yeast_local.tsv output.html CASBJU01 0 200
```