"""
Conversion MP4 → GIF — compatible moviepy v2.x
Dépendances : pip install moviepy Pillow
Usage : python mp4_to_gif.py input.mp4 output.gif [--fps 12] [--width 480] [--start 0] [--end 10]
"""

import argparse
from pathlib import Path
from moviepy import VideoFileClip


def mp4_to_gif(
    input_path: str,
    output_path: str,
    fps: int = 12,
    width: int | None = None,
    start: float | None = None,
    end: float | None = None,
):
    src = Path(input_path)
    if not src.exists():
        raise FileNotFoundError(f"Fichier introuvable : {input_path}")

    clip = VideoFileClip(str(src))

    if start is not None or end is not None:
        end_t = min(end, clip.duration) if end is not None else clip.duration
        clip = clip.subclipped(start or 0, end_t)

    if width is not None:
        clip = clip.resized(width=width)

    print(f"Durée      : {clip.duration:.1f}s")
    print(f"Résolution : {int(clip.w)}x{int(clip.h)}")
    print(f"FPS        : {fps}")
    print("Conversion en cours...")

    clip.write_gif(output_path, fps=fps)
    clip.close()

    output_size = Path(output_path).stat().st_size
    print(f"GIF généré : {output_path} ({output_size / 1024:.1f} Ko)")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Convertit un MP4 en GIF.")
    parser.add_argument("input", help="Chemin du fichier MP4 source")
    parser.add_argument("output", help="Chemin du GIF de sortie")
    parser.add_argument("--fps", type=int, default=12)
    parser.add_argument("--width", type=int, default=None)
    parser.add_argument("--start", type=float, default=None)
    parser.add_argument("--end", type=float, default=None)
    args = parser.parse_args()

    mp4_to_gif(
        args.input, args.output,
        fps=args.fps, width=args.width,
        start=args.start, end=args.end,
    )