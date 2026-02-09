import * as sim from "../app/simulation-wasm/pkg/";

const simulation = new sim.Simulation();
const world = simulation.world();

for (const animal of world.animals) {
  console.log(animal.x, animal.y);
}

const viewport = document.getElementById("viewport");
const ctxt = viewport.getContext("2d");

ctxt.fillStyle = "rgb(0, 0, 0)";

CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size, rotation) {
        this.beginPath();
        this.moveTo(x - Math.sin(rotation) * size, y + Math.cos(rotation) * size);
        this.lineTo(x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size, y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size);
        this.lineTo(x - size, y + size);
        this.lineTo(x, y);

        this.fillStyle = 'rgb(0, 0, 0)';
        this.fill();
    };

for (const animal of world.animals) {
    ctxt.drawTriangle(
        animal.x * viewport.width,
        animal.y * viewport.height,
        0.01 * viewport.width,
        animal.rotation
    );
}
