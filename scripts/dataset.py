import os
import random

TEAM_SIZE = int(os.getenv("TEAM_SIZE", "5"))
N_ROWS = int(os.getenv("N_ROWS", 1000))
RATING_UPPER_LIMIT = int(os.getenv("RATING_UPPER_LIMIT", 30_000))
OUTPUT_FILE = os.getenv("OUTPUT_FILE", "dataset.txt")


def generate_file(
    filename: str, n_lines: int = N_ROWS, max_normal: int = RATING_UPPER_LIMIT
):
    with open(filename, "w") as f:
        for i in range(n_lines):
            print(f"\rGenerating {i + 1}/{n_lines}", end="")

            line = []
            for _ in range(TEAM_SIZE):
                num = random.randint(0, max_normal)
                line.append(str(num))

            _ = f.write(" ".join(line) + "\n")

        print("")


if __name__ == "__main__":
    generate_file(OUTPUT_FILE)
