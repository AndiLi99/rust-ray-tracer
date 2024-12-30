# Rust Ray Tracer
A follow through of https://raytracing.github.io/books/RayTracingInOneWeekend.html in rust.

![Final rendering with 3 large balls and many small balls made of glass, metal and solid materials.](results/final_render.png)

## Parallelized with Rayon
The final render above was 14x faster from 38 minutes to 3 minutes. [Rayon](https://docs.rs/rayon/latest/rayon/) is easy to use.

## Process
Here's a gif of the progression of renders showing the ray tracer's development over time.
![Progression of renders during the ray tracer's development, resulting in the final render.](results/part_one.gif)