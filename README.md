# What is this project?
This is a website developed for planning autonomous paths for FRC robots.
It is intended to be a simple and fast tool to create autos, which works offline at competitions. 

# How it works
Autonomous programs are defined in this app as a series of points (as a linear spline), where each point has a custom rotation value. 

The rotation feature is available for holonomic drive trains (such as swerve) that can separate the direction of the drivetrain from the direction of travel.

After generating a path (keys and features below in Usage), copy the path as json from the text box at the top right.

Note: the points are all in field position as opposed to being pixel values on the screen. 

# Usage 
### Menu
`i` - Toggles the menu at the top right (with the clear button and json export box)

### Point controls
`Space` or `Left Click` - Add a point at the mouse cursor position (field position is displayed in the box below the cursor)

`z` - Remove the selected point (the selected point is highlighted gray)

`r` - Rotate the selected point towards the mouse cursor (angle shown visually and above the cursor numerically)

`e` - "Edit" (Move the selected point to the current mouse position)

`a` and `d` - `a` selects the previous point, while `d` selects the next point

`c` - Splits the selected point into two (doesn't work on edge points)

# Development
This app is built with rust and macroquad (compiled to WASM). I originally created this project as a simple project to learn rust with, but I decided to flesh it out.
Because of this, it is likely that there are various possible improvements to my code.

I am also currently considering switching away from macroquad because of the large package size.

### Testing and building the code
First, clone the repository locally, and install rust (I recommend using the Visual Studio tools method).

Next, add WASM as a target for rust (assuming rust is installed)
```
rustup target add wasm32-unknown-unknown
```

Next, to build it in development mode, run 

```
cargo build --target wasm32-unknown-unknown
```

Then, to see it working, run a local server hosting the index.html file in the main directory.

The codebase is currently setup to automatically build using a github action, so if you want to build it completely locally you will need to run

```
cargo build --release --target wasm32-unknown-unknown
```

and then copy the field image into `dist/assets` and the wasm file from `target/wasm32-unknown-unknown/release/frc-auto-generator.wasm` to `dist/frc-auto-generator.wasm`.
