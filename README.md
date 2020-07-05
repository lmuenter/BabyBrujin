# BabyBrujin

![Project Image](/icons/BabyBrujin_icon.png)

> Assemble simple sentences from gibberish
> `version: 0.0.15 - "Newborn"`

### Table of Contents

- [Description](#description)
- [How To Use](#how-to-use)
- [Caveats](#caveats)

## Description
This Package contains all functions that can be used to build BabyBrujin:
the gibberish-assembler to end all gibberish assemblers.

#### Dependencies
The following R-packages should be installed:

- magrittr
- stringr

## How To Use

#### Installation
install.github("lmuenter/BabyBrujin")

#### Basic Usage
1. Type in a sentence: 

`sentence = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.`

2. Build some reads: 

`reads = get_kmers(sentence, k+4) %>% sample(., 200, replace = TRUE)`
	
3. Select Nodes 

`nodes = get_nodes(reads = reads, k)`
	
4. Generate the graph by overlaps of left and right k-1-mers: 

`OurGraph = get_graph(nodes)`

5. Walk the graph: 

`path = walk_the_graph(OurGraph)`
	
6. Build based on the graph: 

`contigs = get_contigs(path, nodes)`

## Caveats
- Does not handle repetition at all. A Depth-First-Search (DFS) will be integrated lateron.
- Does only assemble Sentences. Sequence Assembly of reads in .fastq-format could be added lateron