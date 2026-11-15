#!/usr/bin/env python3
"""Draws the PisiWorks icon as PNG files, one for each size the desktop asks for.

The same picture as pisiworks.svg, drawn here so that no SVG renderer is needed.
Run from the packaging folder: python3 make-icons.py
"""
from pathlib import Path

from PIL import Image, ImageDraw

SIZES = (16, 32, 48, 64, 128, 256)
WHITE, DARK, ORANGE, YELLOW = (255, 255, 255, 255), (23, 82, 122, 255), (240, 138, 36, 255), (255, 211, 77, 255)


def draw(size):
    # Drawn on a grid of 256 four times enlarged, then scaled down for smooth edges.
    scale = 4
    big = 256 * scale
    at = lambda points: [(x * scale, y * scale) for x, y in points]
    box = lambda cx, cy, rx, ry: [(cx - rx) * scale, (cy - ry) * scale, (cx + rx) * scale, (cy + ry) * scale]
    # A round badge, teal at the top and blue at the bottom.
    gradient = Image.new("RGBA", (big, big))
    pen = ImageDraw.Draw(gradient)
    for y in range(big):
        part = y / (big - 1)
        pen.line([(0, y), (big, y)], fill=tuple(round(a + (b - a) * part) for a, b in zip((31, 163, 163), (23, 82, 122))) + (255,))
    mask = Image.new("L", (big, big), 0)
    ImageDraw.Draw(mask).ellipse(box(128, 128, 120, 120), fill=255)
    image = Image.new("RGBA", (big, big), (0, 0, 0, 0))
    image.paste(gradient, (0, 0), mask)
    pen = ImageDraw.Draw(image)
    # A cat's head: "pisi" is what a cat is called in Turkish.
    pen.polygon(at([(62, 128), (70, 52), (122, 96)]), fill=WHITE)
    pen.polygon(at([(194, 128), (186, 52), (134, 96)]), fill=WHITE)
    pen.ellipse(box(128, 142, 72, 60), fill=WHITE)
    pen.ellipse(box(102, 136, 9, 13), fill=DARK)
    pen.ellipse(box(154, 136, 9, 13), fill=DARK)
    pen.polygon(at([(120, 160), (136, 160), (128, 170)]), fill=ORANGE)
    # A spark, for the works.
    pen.polygon(at([(206, 34), (213, 57), (236, 64), (213, 71), (206, 94), (199, 71), (176, 64), (199, 57)]), fill=YELLOW)
    return image.resize((size, size), Image.LANCZOS)


if __name__ == "__main__":
    out = Path(__file__).parent / "icons"
    out.mkdir(exist_ok=True)
    for size in SIZES:
        draw(size).save(out / f"pisiworks-{size}.png")
        print(f"icons/pisiworks-{size}.png")
