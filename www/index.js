import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();

for (const animal of world.animals) {
  console.log(animal.x, animal.y);
}

const viewport = document.getElementById("viewport");
const ctxt = viewport.getContext("2d");

ctxt.fillStyle = "rgb(0, 0, 0)";

CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size) {
        this.beginPath();
        this.moveTo(x, y);
        this.lineTo(x + size, y + size);
        this.lineTo(x - size, y + size);
        this.lineTo(x, y);

        this.fillStyle = 'rgb(0, 0, 0)';
        this.fill();
    };

ctxt.drawTriangle(50, 0, 50);

for (const animal of simulation.world().animals) {
    ctxt.drawTriangle(
        animal.x * viewportWidth,
        animal.y * viewportHeight,
        0.01 * viewportWidth,
    );
}
