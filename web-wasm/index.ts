import w from './pkg';
import m from "./pkg/index_bg.wasm";

const WIDTH = 640, HEIGHT = 320;
const rat = WIDTH / 64;

async function main() {
    const { get_keys, set_rom, step } = await w;
    const { memory } = await m;
    const keys = get_keys();
    const canvas = document.getElementById('main_canvas') as HTMLCanvasElement;
    canvas.width = WIDTH;
    canvas.height = HEIGHT;
    const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
    const image = new Uint8ClampedArray(memory.buffer, keys.offset(), keys.size());
    const res = await fetch('roms/TETRIS');
    const buffer = await res.arrayBuffer();
    set_rom(new Uint8Array(buffer));
    function draw() {
        requestAnimationFrame(draw);
        step();
        for (let i = 0; i < image.length; i++) {
            ctx.fillStyle = image[i] === 0 ? 'rgb(16, 29, 43)' : '#8F9186';
            const [x, y] = [i % 64 * rat, Math.floor(i / 64) * rat]
            ctx.fillRect(x, y, rat, rat);
        }
        // const imageData = ctx.createImageData(WIDTH, HEIGHT);
        // for (let i = 0; i < image.length; i++) {
        //     const j = i * 4;
        //     // 0 -- 16, 29, 43   1 -- 0x8F, 0x91, 0x86
        //     imageData.data[j] = image[i] === 0 ? 16 : 0x8F;
        //     imageData.data[j + 1] = image[i] === 0 ? 29 : 0x91;
        //     imageData.data[j + 2] = image[i] === 0 ? 43 : 0x86;
        //     imageData.data[j + 3] = 255;
        // }
        // ctx.putImageData(imageData, 0, 0);
    }
    requestAnimationFrame(draw);
}

main();
