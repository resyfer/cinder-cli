import random


def generate_file(filename: str, n_lines: int, max_normal: int = 30000):
    with open(filename, "w") as f:
        for _ in range(n_lines):
            line = []
            for _ in range(5):
                # With small probability, generate an outlier > max_normal
                num = random.randint(0, max_normal)
                line.append(str(num))
            _ = f.write(" ".join(line) + "\n")


if __name__ == "__main__":
    generate_file("dataset.txt", n_lines=100_000_000)
