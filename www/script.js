import init from "./pkg/web.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const wasm = await init("./pkg/web_bg.wasm");
  const { draw_mandelbrot } = wasm;

  let canvas = document.getElementById('display');
  let ctx = canvas.getContext('2d');
  let scaleElem = document.getElementById('scale');
  let simdElem = document.getElementById('simd');
  let iterElem = document.getElementById('max-iter');
  let renderButton = document.getElementById('render');
  let outputElem = document.getElementById('output');

  // Listener for button click
  renderButton.addEventListener('click', () => {
    let scale = parseFloat(scaleElem.value);
    let simd = simdElem.checked;
    let max_iter = parseInt(iterElem.value);
    let width = 800 * scale;
    let height = 600 * scale;
    canvas.width = width;
    canvas.height = height;
    let time = draw_mandelbrot(ctx, width, height, simd, max_iter);
    time = Math.ceil(time.duration);
    outputElem.textContent = `${time}ms`;
  });

};
runWasm();