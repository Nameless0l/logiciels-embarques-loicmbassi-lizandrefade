"""
Compression de GIF via réduction des couleurs et optimisation des frames.
Dépendances : pip install Pillow
Usage : python compress_gif.py input.gif output.gif [--colors 64] [--scale 0.75]
"""

import argparse
from pathlib import Path
from PIL import Image, ImageSequence


def compress_gif(input_path: str, output_path: str, colors: int = 64, scale: float = 1.0):
    src = Path(input_path)
    if not src.exists():
        raise FileNotFoundError(f"Fichier introuvable : {input_path}")

    with Image.open(src) as img:
        frames = []
        durations = []

        for frame in ImageSequence.Iterator(img):
            f = frame.convert("RGBA")

            if scale != 1.0:
                new_size = (int(f.width * scale), int(f.height * scale))
                f = f.resize(new_size, Image.LANCZOS)

            # Quantification des couleurs
            f = f.quantize(colors=colors, method=Image.Quantize.FASTOCTREE)
            frames.append(f)

            duration = frame.info.get("duration", 100)
            durations.append(duration)

        if not frames:
            raise ValueError("Aucune frame trouvée dans le GIF.")

        frames[0].save(
            output_path,
            save_all=True,
            append_images=frames[1:],
            loop=img.info.get("loop", 0),
            duration=durations,
            optimize=True,
        )

    original_size = src.stat().st_size
    compressed_size = Path(output_path).stat().st_size
    ratio = (1 - compressed_size / original_size) * 100
    print(f"Original  : {original_size / 1024:.1f} Ko")
    print(f"Compressé : {compressed_size / 1024:.1f} Ko")
    print(f"Gain      : {ratio:.1f}%")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Compresse un fichier GIF.")
    parser.add_argument("input", help="Chemin du GIF source")
    parser.add_argument("output", help="Chemin du GIF compressé")
    parser.add_argument("--colors", type=int, default=64,
                        help="Nombre de couleurs max (2-256, défaut: 64)")
    parser.add_argument("--scale", type=float, default=1.0,
                        help="Facteur de redimensionnement (ex: 0.75 = 75%, défaut: 1.0)")
    args = parser.parse_args()

    compress_gif(args.input, args.output, colors=args.colors, scale=args.scale)