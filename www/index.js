import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();

for (const animal of world.animals) {
  console.log(animal.x, animal.y);
}
