# What The Particle?! — A LiquidO Game

This project is comprised of two main components:

1. **Mount Charles**, a custom GEANT4-style particle propagation simulation.
2. **What The Particle?!**, an interactive guessing game which uses Mount Charles.

This project was originally created as an outreach tool for the LiquidO experiment, and was partly inspired by the Zooniverse's [Name That Neutrino](https://www.zooniverse.org/projects/icecubeobservatory/name-that-neutrino) project.

Mount Charles is written in pure Rust, while the game renders the results of the simulation using JavaScript (specifically ThreeJS).

# Mount Charles

## Conventions
Mirroring GEANT4, the simulation uses millimetres (mm), nanoseconds (ns) and mega-electronvolts (MeV) as the base units for length, time and energy respectively.

Natural units are used for all the electronvolt-related quantities, so the units of mass, energy and momentum are all MeV. 

## Structure

### World
The top-level object in the simulation is the world. This stores:
1. Global time: the absolute time of the simulation, which starts at 0ns when the particle(s) is produced.
2. Time step: the time step used in the simulation. Each simulation step will advance the global time by this time step.
3. List of particles: a list of all the particles in the event. These are kept in the list even when they are no longer being simulated (due to exiting the volume, decaying, etc.).
4. Volume: the simulation volume in which the particles are contained. The particles are killed upon exiting this volume.

### Volume
The simulation volume is a cube centred on the origin and characterized by a single `size` parameter: this corresponds to the edge length of the simulation cube. Particle [interaction](#interactions) and [propagation](#propagation) is only calculated inside this volume.

### Particle
A particle is made of two components:
1. Particle type: currently one of electron ($e^-$), muon ($\mu^-$) or gamma ($\gamma$).
2. Particle state: this describes the particle's properties, namely
   1. Position: the 3D position of the particle (mm).
   2. Momentum: the momentum of the particle (MeV).
   3. Mass: the mass of the particle (MeV).
   4. Alive: whether the particle is considered "alive" or not, i.e. whether it is still being simulated.
   
## Physics processes

### Propagation
For every time step, every active particle in the simulation is propagated with the simple formula

$$\vec{r} \text{+=} \hat{p}\cdot\beta\cdot c\cdot \Delta t$$

where $\vec{r}$ is the particle position, $\hat{p}$ its normalized momentum vector, $\beta$ the speed parameter of the particle ($v/c$), $c$ the speed of light (in appropriate units, mm/ns) and $\Delta t$ the time step (in ns).

### Interactions
As this is a very simple particle simulator, only the dominant interaction for each particle type is implemented.

#### Electron

# What The Particle
This section describes the game aspect of this project, including the different functionalities and levels available.

## Navigation
The 3D view has two different types of cameras: perspective and orthographic. In orthographic mode, the camera looks along the axis, with the axis in question being determined by the user.

In perspective mode, the view can be zoomed, panned and rotated. In orthographic mode, only zooming and panning are possible.

The navigation operations are summarized in the following table:
| Action                    | Input                         |
| :------------------------ | :---------------------------: |
| Zoom in                   | Scroll up                     |
| Zoom out                  | Scroll down                   |
| Pan                       | `Shift` + `Left click` + drag |
| Rotate                    | `Left click` + drag           |
| Perspective view          | `0` or `p`                    |
| Orthographic view along x | `1` or `x`                    |
| Orthographic view along y | `2` or `y`                    |
| Orthographic view along z | `3` or `z`                    |

# Development

## Compiling to JS
In order to expose an API usable from JS, the `wasm_bindgen` crate is used. In the top-level `lib.rs` file, a `WASMWorld` struct is created, annotated with `#[wasm_bindgen]`. The `WASMWorld` struct can then be `impl`-ed , adding any API that is desired.

In order to compile this, run (in the root directory of the project, next to `Cargo.toml`):
``` zsh
wasm-pack build --target web -m no-install
```
(note that the installed `wasm-bindgen` and the cargo `wasm-bindgen` versions must match exactly).

This creates a `pkg/` which contains a `mount_charles.js` file. This can be imported in JS:
``` zsh
import init, { WASMWorld } from "./pkg/mount_charles.js";
```
in order to use the functions exposed by the API.
