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
The simulation volume is a cube centred on the origin and characterized by a single `size` parameter: this corresponds to the edge length of the simulation cube. Particle [interaction](#interactions) and [propagation](#propagation) is only calculated inside this volume. For simplicity, the volume is taken to be made of liquid water.

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
For every time step, every active particle in the simulation is propagated with the small position update

$$\Delta\vec{r} = \hat{p}\cdot\beta c\cdot \Delta t$$

where $\vec{r}$ is the particle position, $\hat{p}$ its normalized momentum vector, $\beta$ the speed parameter of the particle ($v/c$), $c$ the speed of light (in appropriate units, mm/ns) and $\Delta t$ the time step (in ns).

### Interactions
As this is a very simple particle simulator, only the dominant interaction for each particle type is implemented.

#### Electron
Electron interactions are treated in two parts: energy loss and scattering. These are essentially the manifestations of the change in momentum at each time step: energy loss corresponds to the change in the momentum magnitude, while scattering corresponds to the change in the momentum direction.

**Energy loss:** Energy loss for ionizing particles is governed by the stopping power of the target material for that particle, notated dE/dx, which varies as a function of particle momentum. For electrons in this simulation, the [NIST ESTAR database](https://physics.nist.gov/PhysRefData/Star/Text/ESTAR.html) is used, with the material set to liquid water. This yields a list of discrete kinetic energy values with their corresponding dE/dx values. In order to obtain an analytical form that can be computed quickly during the simulation, a function (see [Table of coefficients](#table-of-coefficients)) is fit to the database values. Then, at every time step, the energy of the particle is decreased by

$$\Delta E = \frac{dE}{dx}\cdot dx = \frac{dE}{dx}\cdot\beta c\cdot\Delta t$$

Once the electron momentum drops below 0.103 MeV, it is out of the tabulated range, so a constant 8 MeV/cm energy loss is assumed.

**Scattering:** The dominant process in electron scattering (at MeV energies) is multiple Coulomb scattering (MCS). This process can be approximated using the Highland formula (given in this [PDG review](https://pdg.lbl.gov/2019/reviews/rpp2018-rev-passage-particles-matter.pdf) for example), which gives the RMS scattering angle for the electron for a small step $dx$:

$$\theta_0 = \frac{13.6\text{ MeV}}{\beta p}\sqrt{\frac{dx}{X_0}}\left[1 + 0.038\ln{\left(\frac{dx}{X_0}\right)}\right]$$

where $p$ is the momentum of the electron, $\beta$ the speed parameter and $X_0$ the radiation length of the material. In this formula, the $\frac{z^2}{\beta^2}$ term has been omitted from the logarithm, as it is considered to be negligible. In this simulation, the value for water of $X_0=36.08\text{ cm}$ is used.

#### Muon
Muons are treated very similarly to electrons. The two components of the interaction are described below.

**Energy loss:** Energy loss for muons is separated into two cases: momentum above and below 50MeV ("high" and "low" momentum).

The dE/dx curve for the high momentum range is computed from the tabulated values in the full version of [this paper](https://pdg.lbl.gov/2023/AtomicNuclearProperties/adndt.pdf). These values are again fit to a degree 8 "log polynomial" (see [Table of coefficients](#table-of-coefficients)) in order to interpolate between the discrete given values.

For the low momentum range, the Bethe-Bloch equation is used:

$$-\frac{dE}{dx} = K\frac{Z_{\text{eff}}}{A}\frac{1}{\beta^2}\left[\frac{1}{2}\ln{\left(\frac{2m_e \beta^2\gamma^2 T_{\text{max}}}{I^2}\right)}-\beta^2-\frac{C}{Z}-\delta\right]$$

where $Z_{\text{eff}}$ and $A$ are the effective atomic number and mass of the target, $T_{\text{max}}$ the maximum kinetic energy transfer to an electron, $I$ the mean excitation energy, $\frac{C}{Z}$ the shell correction and $\delta$ the density effect correction which we take to be negligible. Further, $T_{\text{max}}$ is given by (in natural units)

$$T_{\text{max}} = \frac{2m_e\beta^2\gamma^2}{1 + 2\gamma m_e/m_\mu + (m_e/m_\mu)^2}$$

whereas the shell correction is given by (from [this paper](https://pdg.lbl.gov/2023/AtomicNuclearProperties/adndt.pdf))

$$\frac{C}{Z} \approx 0.42237\cdot\left(\frac{1}{\beta\gamma}\right)^2 + 0.0304\cdot\left(\frac{1}{\beta\gamma}\right)^4$$

and the values of the other parameters for liquid water are shown in this table:

| Parameter        | Value    |
| :--------------- | -------: |
| $Z_{\text{eff}}$ | 10       |
| $A$              | 18 g/mol |
| $I$              | 75 eV    |

As for electrons, once the muon momentum drops below 8.9 MeV, it is out of the tabulated range, so a constant 8 MeV/cm energy loss is assumed.

**Scattering:** Muons are scattering identically to electrons. Generally, their higher momentum will mean that their tracks are "straighter".

#### Table of coefficients
The function that is used to recreate the dE/dx curves for electrons and muons is the so-called "log polynomial" of degree $D$, given by

$$\frac{dE}{dx}(p)=\sum_{n=0}^D c_n\ln{(p)}^n$$

For this simulation, the log polynomial of degree 8 is used. The optimal coefficients for each particle are shown in the table below.

| Coefficients | Electrons | Muons (<50 MeV) | Muons (>50 MeV) |
| :----------: | --------: | --------------: | --------------: |
| $c_0$ | $1.97185875\cdot 10^0$     | $-2.21192313\cdot 10^5$ | $1.13754387\cdot 10^3$     |
| $c_1$ | $-4.90322067\cdot 10^{-1}$ | $4.16349323\cdot 10^5$  | $-1.13642381\cdot 10^3$    |
| $c_2$ | $5.67984147\cdot 10^{-1}$  | $-3.02334049\cdot 10^5$ | $4.96588219\cdot 10^2$     |
| $c_3$ | $-3.78515229\cdot 10^{-1}$ | $9.22330794\cdot 10^4$  | $-1.23563655\cdot 10^2$    |
| $c_4$ | $1.96937857\cdot 10^{-1}$  | $1.78846389\cdot 10^3$  | $1.91190645\cdot 10^1$     |
| $c_5$ | $-6.69875048\cdot 10^{-2}$ | $-9.81957228\cdot 10^3$ | $-1.88126582\cdot 10^0$    |
| $c_6$ | $1.30714285\cdot 10^{-2}$  | $2.97223872\cdot 10^3$  | $1.14850292\cdot 10^{-1}$  |
| $c_7$ | $-1.31646064\cdot 10^{-3}$ | $-3.90203242\cdot 10^2$ | $-3.97495919\cdot 10^{-3}$ |
| $c_8$ | $5.29555090\cdot 10^{-5}$  | $1.99344973\cdot 10^1$  | $5.96940644\cdot 10^{-5}$  |

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
