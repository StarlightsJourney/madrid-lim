#!/bin/sh
# Turns editor uploads in assets/uploads into web-ready files in assets/optimized.
# Photos become WebP (rotated upright, at most 1200 px, metadata such as GPS removed).
# Videos become 432 by 960 H.264 MP4 with faststart, plus a WebP poster frame.
# Trunk runs this before every build. Unchanged files are skipped.
set -eu

cd "${TRUNK_SOURCE_DIR:-$(dirname "$0")/..}"
src=assets/uploads
out=assets/optimized
mkdir -p "$src" "$out"

need() {
  for tool in "$@"; do
    if command -v "$tool" >/dev/null 2>&1; then
      printf '%s' "$tool"
      return
    fi
  done
  printf 'optimize-media: %s is needed to process uploads. Install ImageMagick and ffmpeg.\n' "$1" >&2
  exit 1
}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
: > "$tmp/expected"

find "$src" -type f ! -name '.*' | sort | while IFS= read -r file; do
  rel=${file#"$src"/}
  stem=${rel%.*}
  ext=$(printf '%s' "${rel##*.}" | tr '[:upper:]' '[:lower:]')
  mkdir -p "$out/$(dirname "$rel")"
  case "$ext" in
    jpg | jpeg | png | webp | heic | heif)
      target="$out/$stem.webp"
      printf '%s\n' "$target" >> "$tmp/expected"
      [ -f "$target" ] && [ "$target" -nt "$file" ] && continue
      input=$file
      if [ "$ext" = heic ] || [ "$ext" = heif ]; then
        for heif in heif-dec heif-convert; do
          if command -v "$heif" >/dev/null 2>&1; then
            "$heif" "$file" "$tmp/heif.png" >/dev/null
            input=$tmp/heif.png
            break
          fi
        done
      fi
      magick=$(need magick convert)
      "$magick" "$input" -auto-orient -strip -resize '1200x1200>' -quality 80 "$target"
      printf 'optimize-media: %s -> %s\n' "$file" "$target"
      ;;
    mp4 | mov | m4v)
      target="$out/$stem.mp4"
      poster="$out/$stem-poster.webp"
      printf '%s\n%s\n' "$target" "$poster" >> "$tmp/expected"
      [ -f "$target" ] && [ "$target" -nt "$file" ] && [ -f "$poster" ] && continue
      ffmpeg=$(need ffmpeg)
      "$ffmpeg" -nostdin -y -v error -i "$file" -map 0:v:0 -map '0:a:0?' -map_metadata -1 \
        -vf 'scale=w=432:h=960:force_original_aspect_ratio=decrease:force_divisible_by=2' \
        -c:v libx264 -preset slow -crf 26 -profile:v high -pix_fmt yuv420p \
        -c:a aac -b:a 64k -ac 2 -movflags +faststart "$tmp/video.mp4"
      mv "$tmp/video.mp4" "$target"
      rm -f "$tmp/poster.png"
      "$ffmpeg" -nostdin -y -v error -ss 1 -i "$target" -frames:v 1 "$tmp/poster.png" || true
      [ -s "$tmp/poster.png" ] || "$ffmpeg" -nostdin -y -v error -i "$target" -frames:v 1 "$tmp/poster.png"
      webp=$(need magick convert cwebp)
      if [ "$webp" = cwebp ]; then
        cwebp -quiet -q 80 "$tmp/poster.png" -o "$poster"
      else
        "$webp" "$tmp/poster.png" -quality 80 "$poster"
      fi
      printf 'optimize-media: %s -> %s\n' "$file" "$target"
      ;;
    *)
      printf 'optimize-media: skipping %s, unsupported file type\n' "$file" >&2
      ;;
  esac
done

find "$out" -type f | while IFS= read -r file; do
  grep -qxF "$file" "$tmp/expected" || rm -f "$file"
done
