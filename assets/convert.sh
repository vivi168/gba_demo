#!/bin/sh

# Credit to DawnBringer / Dawnlike
# https://opengameart.org/content/dawnlike-16x16-universal-rogue-like-tileset-v181

superfamiconv -M gba -i bg.png -p bg.pal -t bg.tiles -m bg.map --color-zero 140C1C -v
superfamiconv -M gba -i char.png -p char.pal -t char.tiles --color-zero ffffff -v
