import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();

for (const animal of world.animals) {
  console.log(animal.x, animal.y);
}

const viewport = document.getElementById("viewport");
const ctxt = viewport.getContext("2d");

ctxt.fillStyle = "rgb(0, 0, 0)";

for (const animal of simulation.world().animals) {
    ctxt.fillRect(animal.x * viewport.width, animal.y * viewport.height, 15, 15);
}
