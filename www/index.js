import * as sim from "../app/simulation-wasm/pkg";

const simulation = new sim.Simulation();
const viewport = document.getElementById("viewport");
const ctxt = viewport.getContext("2d");

ctxt.fillStyle = "rgb(0, 0, 0)";

const forward_scale = 1.5;
CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size, rotation) {
        this.beginPath();

        this.moveTo(
            x - Math.sin(rotation) * size * forward_scale,
            y + Math.cos(rotation) * size * forward_scale,
        );

        this.lineTo(
            x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x - Math.sin(rotation) * size * forward_scale,
            y + Math.cos(rotation) * size * forward_scale,
        );

        this.stroke();
    };

function redraw() {
    ctxt.clearRect(0, 0, viewport.width, viewport.height);

    simulation.step();

    for (const animal of simulation.world().animals) {
        ctxt.drawTriangle(
            animal.x * viewport.width,
            animal.y * viewport.height,
            0.01 * viewport.width,
            animal.rotation,
        );
    }
    requestAnimationFrame(redraw);
}

redraw();
