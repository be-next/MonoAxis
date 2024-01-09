# MonoAxis

![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/be-next/MonoAxis?logo=github)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/be-next/MonoAxis/rust.yml?logo=github)
![GitHub License](https://img.shields.io/github/license/be-next/MonoAxis?logo=apache)
![GitHub top language](https://img.shields.io/github/languages/top/be-next/MonoAxis?logo=rust)

MonoAxis is a One-dimensional Cellular Automaton tool.

In this repository, you will find a Rust library and a CLI tool to play with 1D CA.
In the future, I plan to add a GUI tool to make it more fun to play with.

## Table of Contents


  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Usage](#usage)
  - [How does it work?](#how-does-it-work)
  - [Some Examples](#some-examples)
  - [Cellular Automata in a Nutshell](#cellular-automata-in-a-nutshell)
  - [Why a 1D Cellular Automaton nowadays? :thinking:](#why-a-1d-cellular-automaton-nowadays-thinking)
    - [For the Fun of It](#for-the-fun-of-it)
    - [Simplicity and Dynamics Understanding](#simplicity-and-dynamics-understanding)
    - [Learning Rust Programming](#learning-rust-programming)


## Getting Started

### Prerequisites

MonoAxis is written in Rust, and you will need to install the Rust toolchain to build and run it.
So, to build and run this project, you will need the following tools (as usual):

- [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (no matter the OS)

- [Git](https://git-scm.com/downloads) or whatever you want to clone the repo.


### Installation

Nothing fancy here, just clone the repo and build the project.

1. Clone the repo
   ```git clone https://github.com/be-next/MonoAxis.git```
2. Build the project
   ```cargo build```
3. Run the project

### Usage

In the ```examples``` directory, you will find examples of 1D CA rules.
You can use them to play with the CLI tool.

For example, to run the ```Sum``` rule, you can use the following command:
```cargo run -- -c examples/example_01/configuration.json -s 7```  

Comprehensive description of ```mano_axis_cli``` is here: [mano_axis_cli](cli/README.md).

## How does it work?

### Cellular Automaton

MoNoAxis cellular automaton is composed of a 1D array of cells.
Each cell can be in one of several states. The number of states is configurable.
The state of each cell changes over discrete time according to predefined rules,
based on the states of the cell and neighboring left and right cells.

The first and last cells of the array are considered neighbors, but never change their state.
It's a convenient way to simulate a closed universe.

### Rules

The rules are defined in a JSON file. Here is an example of a rules:

```json
{
  "name": "Sum",
  "num_states": 3,
  "rules": [
    {"neighborhood": [1, 0, 1], "next_state": 2},
    {"neighborhood": [1, 2, 1], "next_state": 1},
    {"neighborhood": [2, 1, 1], "next_state": 2},
    {"neighborhood": [1, 2, 0], "next_state": 0},
    {"neighborhood": [2, 1, 0], "next_state": 2}
  ]
}
```

Where:
  - The ```name``` is the name of the rule. It's used to identify the rule in the CLI tool.
  - The ```num_states``` is the number of states of the cells. It's used to initialize the lookup table.
  - The ```rules``` is an array of rules. Each rule is composed of a ```neighborhood``` and a ```next_state```.
    - The ```neighborhood``` is an array of the states of the left, current and right cells.
    - The ```next_state``` is the state of the current cell at the next time step.

> [!TIP]
> You only have to define the rules for the states that change.
> If a state is not defined, that means that the cell will keep its current state at the next time step.
> 
> It's a convenient way to define rules for a large number of states without having to define all the rules.

### Configuration

The configuration is defined in a JSON file. Here is an example of a configuration:

```json
{
  "num_states": 3,
  "num_cells": 14,
  "world_initialisation" : "0 0 1 1 1 1 0 1 1 1 1 1 0 0",
  "rules_file_name": "rules.json"
}
```

Where:
  - The ```num_states``` is the number of states of the cells. It's used to initialize the lookup table.
  - The ```num_cells``` is the number of cells in the array.
  - The ```world_initialisation``` is the initial state of the cells. It's a string of space-separated integers.
  - The ```rules_file_name``` is the name of the file containing the rules.


## Some Examples

Here are some examples of 1D CA rules. You can find them in the ```examples``` directory.

  - **example_01**: Sum #1. At the beginning, there are two packets of cells in state 1. The rule is designed to make them move towards each other and merge into one packet.
  - **example_02**: Sum #2. At the beginning, there are two packets of cells in state 1. The rule is designed to show a third packet on the right, which is the sum of the first two packets.
  - **example_04**: Inverse. The rule is designed to invert two packets of cells in state 1.
  - **example_06**: Divide by 2. The rule is designed to divide a packet of cells in state 1 by 2.
  - **example_07**: Generator n+1. The rule is designed to generate a packet of cells in state 1 with a size of n+1.
  - **rule_184**: Rule 184 is a one-dimensional binary cellular automaton rule, 
  notable for solving the majority problem as well as for its ability to simultaneously describe several, 
  seemingly quite different, particle systems: traffic flow, particle deposition, and so on.
  (cf. [Wikipedia](https://en.wikipedia.org/wiki/Rule_184)).


---
## Cellular Automata in a Nutshell

> _A pixelated universe where simple local rules orchestrate worlds of astonishing complexity._

Cellular automata are mathematical models composed of grids of cells, 
each existing in one of several possible states. 
The state of each cell changes over time according to predefined rules, 
typically influenced by the states of neighboring cells. 
These models, simple in design yet capable of demonstrating complex behaviors, 
are utilized in various fields to simulate and explore dynamic systems, 
from biological patterns to computational processes and physical phenomena. 
They offer a fascinating glimpse into how complex structures and behaviors can emerge from simple, 
local interactions.

There are an astronomical number of resources on cellular automata on the web.
But for a simple and clear presentation of cellular automata, 
read chapter 7 of [Daniel Shiffman](https://en.wikipedia.org/wiki/Daniel_Shiffman)'s 
[The Nature of Code](https://natureofcode.com/book/chapter-7-cellular-automata/).

## Why a 1D Cellular Automaton nowadays? :thinking:

### For the Fun of It
One-dimensional cellular automata (1D CA) are just cool to play with.
They're visually engaging and offer an intuitive way to get your head around some pretty complex ideas.
Watching these patterns evolve over time is not only fascinating but can also be super fun.
It's a great way to make learning about automata theory a bit more exciting.

### Simplicity and Dynamics Understanding
Compared to their multi-dimensional cousins, 1D CAs are way easier to grasp and visualize.
But don't let their simplicity fool you – they can model some pretty intricate behaviors.
Even with simple rules, 1D CAs can show emergent behaviors and interesting patterns, 
giving us a peek into how simple rules can lead to complex dynamics. 
This makes them perfect for teaching and understanding the chaos theory, 
dynamical systems theory, and computational properties.

### Learning Rust Programming
Approaching one-dimensional cellular automata is for me a way to challenge myself with Rust.
It's less about delving into the specific details of the language and more about using this 
project as a field of exploration to get a handle on Rust. By working on creating a 1D CA, 
I'm giving myself the opportunity to practically test Rust, 
to understand how it works and how it can be used to materialize such an interesting concept. 
It's more of a learning journey and discovery with the language through a project that interests me.
