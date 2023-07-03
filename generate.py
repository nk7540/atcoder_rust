#!/usr/bin/env python3

# from: https://github.com/paruma/atcoder_rust/blob/main/contest.py

# example
# ./contest.py abc206 b c

from __future__ import annotations
import sys
import os
import shutil

if len(sys.argv) <= 2:
    print("usage  : ./contest.py [contest_name] [task_name_list]", file=sys.stderr)
    print("example: ./contest.py abc206 b c", file=sys.stderr)
    sys.exit(1)


contest_name: str = sys.argv[1]
problems: list[str] = sys.argv[2:]


# ソースコードの準備(ディレクトリの生成)
dir_path: str = f"src/contest/{contest_name}/"
if not os.path.exists(dir_path):
    os.mkdir(dir_path)


# ソースコードの準備(ファイルのコピー)
templete_file_path: str = "src/contest/template.rs"
for problem in problems:
    dst_file_path = f"src/contest/{contest_name}/{problem}.rs"
    if not os.path.exists(dst_file_path):
        shutil.copy(templete_file_path, dst_file_path)


# cargo.tomlコードの出力
toml = open("Cargo.toml", "a")
for problem in problems:
    toml.write(
        f"""[[bin]]
name = "{contest_name.replace("/", "")}_{problem}"
path = "src/contest/{contest_name}/{problem}.rs"
    """
    )
