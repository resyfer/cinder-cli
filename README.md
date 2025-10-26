# cinder-cli
A fast and fair rating-based match-making system.

NOTE: This repo is made in the way you would expect to get experimental results. Please check the
[cinder](https://github.com/resyfer/cinder) repo to get the http service, which is of more use.

## Prepare dataset

```sh
cd scripts
N_ROWS=100000 OUTPUT_FILE=dataset.txt python dataset.py
cd ..
```

## Prepare the score outputs

```sh
cargo install --path .
cargo run -- -d scripts/dataset.txt -s scripts/scores.txt
```

## Prepare the graph plots

```sh
cd scripts
python plot_score.py
cd ..
```
