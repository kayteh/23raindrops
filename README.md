# 23raindrops

wip music visualizer thing...

## Basic Usage

```
cargo run -- --input my-cool-music.wav --output my-cool-music.mp4 --map my-cool-music.png
```

## Pipeline Design

1. **Beatmapping (WIP)**

   1. Load a WAV file
   1. Downmix to left, right, or mono (default)
   1. Do 60(?) windowed FFTs per second of music
   1. Do a full FFT of the music, and identify 16 major frequency windows (interpolators)
   1. Map windowed FFTs to interpolators, floating from 0-1 in intensity.

1. **Encoding**

   1. From the beatmap interpolator outputs, create a 512x512 RGBA texture with the following parameters
      - For every group of 16 interpolators, map into 4 RGBA pixels.
      - For every block of 4 pixels, draw a 2x2 square of pixels
      - This allows for 65535 interpolator blocks, representing (60/1000)ms of music each, for a grand total of about 18 minutes of music per texture.
   1. (optional) Output to file for inspection or presentation.

1. **Rendering (WIP)**

   1. Create a headless renderer at 1280x1280 (to fit within 720p in both horizontal and vertical)
   1. Draw 2 triangles into a quad plane, with either edge reaching edges of the window.
   1. Run shader from `src/glsl/fragment.glsl` (and it's dummy vertex shader) sampling individual texels/pixels of the encoded beatmap.
   1. For every 15 seconds of frames (900 frames) stored in memory, flush them to disk as PNG or WebP lossless images; and flush once more at the end.

1. **Presentation (TBD)**

   1. Using ffmpeg, render a video using the frames on disk and the audio.
