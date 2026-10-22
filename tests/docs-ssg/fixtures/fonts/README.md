# Local font fixtures

These IBM Plex Sans Text faces exercise author-owned woff2 and ttf inputs. They are test resources, not CLI assets.

- `author-sans.woff2`: unchanged file `packages/plex-sans/fonts/complete/woff2/IBMPlexSans-Text.woff2` from [IBM/plex at 71f558992315a12ca1f06a2215d57962a1992162](https://github.com/IBM/plex/tree/71f558992315a12ca1f06a2215d57962a1992162). Its notice is `LICENSE-IBM-PLEX-WOFF2.txt` from that same package and commit.
- `author-text.ttf`: unchanged `makepad/widgets/resources/IBMPlexSans-Text.ttf` from this repository's recorded Makepad import. Its notice is `LICENSE-IBM-PLEX-TTF.txt`; the original resource and provenance remain unchanged.

Both are licensed under SIL OFL 1.1. Browser CSS registers them under test aliases; the font bytes and internal names are unchanged. No production font is embedded in the CLI by these fixtures.
