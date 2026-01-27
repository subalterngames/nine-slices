from pathlib import Path
import re

src = Path(__file__).parent.joinpath('README.md').read_text()
filenames = ['src', 'sliced_and_scaled', 'scaled']

# Generate the GitHub README.
github = src[:]
# Pretty code.
github = re.sub(r'^(```)((.|\n)*?)^```', r'```rust\2```', github, flags=re.MULTILINE)

for m, f in zip(re.findall(r'^(!\[(.*?)](\(\)))', github, flags=re.MULTILINE),
                filenames):
    github = github.replace(m[0], f'![{m[1]}](doc/images/{f}.png)')
Path(__file__).parent.parent.joinpath('README.md').write_text(github)

# Generate the Rust README.
rust = src[:]
for m, f in zip(re.findall(r'^(!\[(.*?)](\(\)))', rust, flags=re.MULTILINE),
                filenames):
    rust = rust.replace(m[0], f'![{m[1]}][{f}]')
Path(__file__).parent.joinpath("README-rust.md").write_text(rust)