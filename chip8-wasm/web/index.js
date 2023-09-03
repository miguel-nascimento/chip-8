import init, { Emulator } from "../pkg/chip8_wasm";

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 15;

const BG_COLOR = "#000000";
const PRIMARY_COLOR = "#FFFFFF";

const TICKS_PER_FRAME = 15;
let last_tick = 0;

const romInput = document.getElementById("rom");

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const main = async () => {
  // Wasm module is initialized here
  await init();

  let chip8 = new Emulator();

  document.addEventListener("keydown", (e) => {
    chip8.keypress(e, true);
  });

  document.addEventListener("keyup", (e) => {
    chip8.keypress(e, false);
  });

  romInput.addEventListener(
    "change",
    (e) => {
      // Stop previous game from rendering, if one exists
      if (last_tick != 0) {
        window.cancelAnimationFrame(last_tick);
      }

      let file = e.target.files[0];
      if (!file) {
        return;
      }

      let fileReader = new FileReader();
      fileReader.onload = () => {
        const buffer = fileReader.result;
        const rom = new Uint8Array(buffer);

        chip8.reset();
        chip8.load_rom(rom);
        gameloop(chip8);
      };

      fileReader.readAsArrayBuffer(file);
    },
    false
  );
}

function gameloop(chip8) {
  for (let i = 0; i < TICKS_PER_FRAME; i++) {
    chip8.emulate_cycle();
  }

  chip8.tick_timers();

  ctx.fillStyle = BG_COLOR;
  ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

  ctx.fillStyle = PRIMARY_COLOR;
  chip8.draw(SCALE);

  last_tick = window.requestAnimationFrame(() => gameloop(chip8));
}

main().catch(console.error);
