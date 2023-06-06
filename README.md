A calculator for the angle of inclination of a ramp given the acceleration and the coefficient of friction of an object sliding on it.

## How to run it locally

To run this program locally, you need to have Rust and Cargo installed on your machine. You can follow the instructions [here](https://www.rust-lang.org/tools/install) to install them.

Then, you can clone this repo and navigate to the project directory:

```bash
git clone
cd inclinometer
```

Finally, you can run and build the program with Cargo:

```bash
cargo run
```

The program will read the data from the `input.csv` file and write the results to the `output.csv` file.

## Input Format

The input file should be a CSV file with two columns: `a` and `uk`. The `a` column represents the acceleration of the object in m/s², and the `uk` column represents the coefficient of friction. The columns should be separated by semicolons (;). For example:

```csv
a;uk
8.54;0.2
7.53;0.5
7.2;0.6
7.87;0.4
8.87;0.1
```
