import os
import re
from typing import List
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.ticker import FuncFormatter

INPUT_FILE = os.getenv("OUTPUT_FILE", "scores.txt")


def parse_scores(file_path: str) -> List[int]:
    scores = []
    with open(file_path, "r") as file:
        for line in file:
            match = re.search(r"\s*([0-9.]+)", line)
            if match:
                value = float(match.group(1))
                if value >= 0:
                    scores.append(value)
    return np.array(scores)


def plot_log_transformed_histogram(scores: List[int], num_bins=100):
    # Transform the scores to compress the right tail and make it more symmetric
    transformed_scores = np.log1p(scores)  # log(1 + x), safe for 0

    # Histogram in transformed space
    counts, bin_edges = np.histogram(transformed_scores, bins=num_bins)

    bin_centers = 0.5 * (bin_edges[:-1] + bin_edges[1:])

    plt.figure(figsize=(10, 6))
    plt.bar(
        bin_centers,
        counts,
        width=np.diff(bin_edges),
        align="center",
        edgecolor="black",
        alpha=0.75,
    )

    # Label x-axis with original values using inverse transform
    def inv_log1p(x):
        return np.expm1(x)

    def log_tick_formatter(x, _):
        return f"{inv_log1p(x):.0f}"

    plt.gca().xaxis.set_major_formatter(FuncFormatter(log_tick_formatter))
    plt.xlabel("Sanction Score")
    plt.ylabel("Frequency")
    plt.title("Sanction Score vs Frequency")
    plt.grid(True, linestyle="--", alpha=0.5)
    plt.tight_layout()
    plt.show()


if __name__ == "__main__":
    scores = parse_scores(INPUT_FILE)
    plot_log_transformed_histogram(scores)
