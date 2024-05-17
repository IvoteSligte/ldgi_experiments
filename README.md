### Lower-dimensional Global Illumination (LDGI) Experiments
2D experiments to try out global illumination techniques.

Currently only one technique is implemented, which does the following:
- For every pixel, for every color channel,
  find the coordinates of the brightest pixel
  and lerp own color with said pixel's.
  Coords of brightest pixels are propagated with cellular automata.
- Calculate how much an obstacle blocks light sources by checking
  if neighbours have the same coords of the light source stored.
- Apply a small blur on the colors.
