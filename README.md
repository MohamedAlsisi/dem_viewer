# DEM Viewer

A Rust-based application to visualize high-resolution Digital Elevation Maps (DEM) from ASC files using grayscale, color gradients, and hillshading. The application supports generation of detailed terrain visualizations with optional contour lines overlayed for enhanced topographical understanding.

---

## 👨‍💻 Authors

- Mohamed Alsisi  
- Pablo Candelas

---

## 📌 Description

This project is a visualization tool for DEM data based on ASC (ESRI ASCII Raster) files. It includes several powerful features such as:

- Parsing metadata and elevation data from `.asc` files.
- Rendering grayscale elevation maps normalized between minimum and maximum values.
- Applying the **Turbo** color gradient (`colorgrad` crate) to visualize terrain elevation in color.
- Implementing a custom **hillshading algorithm** for realistic terrain illumination based on sun azimuth and altitude.
- Optional rendering of **contour lines** for advanced terrain analysis.
- Saving outputs to PNG files for further inspection and documentation.

---

## 🎨 Features

| Feature              | Description |
|----------------------|-------------|
| ✅ Grayscale DEM      | Normalized from min to max elevation values |
| ✅ Colored DEM        | Using `colorgrad::turbo` for vibrant color mapping |
| ✅ Hillshading        | Simulated lighting based on slope and aspect |
| ✅ Hillshade + Color  | Combined shaded + color image for realism |
| ✅ Contour Lines      | Optional 3x3 red contour overlays for elevation breaks |

---

## 📁 Input

The input is a `.asc` file structured in the ESRI ASCII raster format, for example:

