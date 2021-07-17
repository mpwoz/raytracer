#!/usr/bin/env zsh

# individual ppms
echo "Converting individual .ppm files in ./output"
for ppm in ./output/*.ppm; do
  echo "$ppm";
  outfn=$(basename "$ppm")
  convert "$ppm" ./images/"$outfn".png
done

# the animated clock gif
echo "Converting clock frames to a gif."
[ -d "./output/clockframes/" ] && convert -delay 20 -loop 0 ./output/clockframes/* ./images/clock.gif

