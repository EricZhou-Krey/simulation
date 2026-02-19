# 🧬 Shorelark: Evolution Simulation in Rust

[![Live Demo](https://img.shields.io/badge/demo-live-brightgreen)](https://EricZhou-Krey.github.io/simulation/)

> **🌐 Live Simulation:** [https://EricZhou-Krey.github.io/simulation/](https://EricZhou-Krey.github.io/simulation/)

A neural-network-driven evolution simulation where "birds" learn to navigate and gather food. This project was built while following the **[Learning to Fly in Rust](https://pwy.io/en/posts/learning-to-fly-pt1/)** tutorial series to explore systems programming, WebAssembly, and AI.

---

## 🚀 Project Overview

The simulation uses a **Genetic Algorithm** to evolve the behavior of agents (birds) equipped with a simple **Neural Network**. 

* **Brain:** Each bird has a feed-forward neural network that takes "eye" inputs (distance/angle to food) and outputs movement commands.
* **Evolution:** Successful birds pass their weights to the next generation through selection, crossover, and mutation.
* **Performance:** The simulation engine is written in **Rust**, compiled to **WebAssembly (WASM)**, and rendered in the browser for high-performance execution.

---

## 🧠 Learning Outcomes

By completing this tutorial and building this repository, I gained hands-on experience in:

* **Rust Memory Management:** Implementing ownership, borrowing, and interior mutability patterns (like `RefCell`).
* **WebAssembly (WASM):** Using `wasm-bindgen` to bridge the gap between high-performance Rust logic and a JavaScript frontend.
* **Genetic Algorithms:** Implementing the core cycle of **Selection -> Crossover -> Mutation**.
* **Linear Algebra for Games:** Calculating vectors for "field of view," distances, and agent rotation.
* **Workspace Architecture:** Structuring a multi-crate Rust project separating the engine, the GA logic, and the web interface.

---

## 🛠️ Project Structure

* `/shorelark`: The core library containing the math and simulation engine.
* `/app`: The implementation of the genetic algorithm and neural network.
* `/www`: The web-based frontend that renders the simulation via WASM and JavaScript.

---
