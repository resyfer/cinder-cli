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
RUST_LOG=debug cargo run -- -d scripts/dataset.txt -s scripts/scores.txt
```

NOTE: The program take the first line of the dataset as the lobby of players, henceforth called searcher lobby, wishing to find their "sanction score" against the other lobbies, henceforth called the waiting lobbies. The sanction score quantifies the disparity between the searcher lobby and waiting lobbies. It then outputs the top 5 matches for the searcher lobby among the waiting lobbies.

## Prepare the graph plots

```sh
cd scripts
python plot_score.py
cd ..
```

<img width="995" height="594" alt="image" src="https://github.com/user-attachments/assets/6785fa06-e3df-4d95-b9c8-edf89e22fbab" />
