# Rust Ray Tracer
A follow through of https://raytracing.github.io/books/RayTracingInOneWeekend.html in rust.

Notes
- How can light bouncing between two colored objects with no absorption be modeled, without making the light darker?

Ideas
- Use a different color model besides RGB
  - Perceptual color models
  - Subtractive color models for materials
  - HSL / HSV for specifiying color