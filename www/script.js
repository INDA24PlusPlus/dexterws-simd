import init from "./pkg/web.js";

const canvas = document.getElementById('display');
const ctx = canvas.getContext('2d');
const scaleElem = document.getElementById('scale');
const simdElem = document.getElementById('simd');
const iterElem = document.getElementById('max-iter');
const renderButton = document.getElementById('render');
const outputElem = document.getElementById('output');

const realMinElem = document.getElementById('min-r');
const realMaxElem = document.getElementById('max-r');
const imagMinElem = document.getElementById('min-i');
const imagMaxElem = document.getElementById('max-i');

const redElem = document.getElementById('red');
const greenElem = document.getElementById('green');
const blueElem = document.getElementById('blue');

const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);

const copyLinkButton = document.getElementById('share');

const loadParams = () => {
  scaleElem.value = urlParams.get('scale') || 1;
  simdElem.checked = urlParams.get('simd') === 'true';
  iterElem.value = urlParams.get('max_iter') || 1000;
  realMinElem.value = urlParams.get('min_r') || -2;
  realMaxElem.value = urlParams.get('max_r') || 0.47;
  imagMinElem.value = urlParams.get('min_i') || -1.12;
  imagMaxElem.value = urlParams.get('max_i') || 1.12;
  redElem.value = urlParams.get('red') || 196;
  greenElem.value = urlParams.get('green') || 16;
  blueElem.value = urlParams.get('blue') || 96;
}

loadParams();

const updateLink = () => {
  let url = window.location.href.split('?')[0];
  url += `?scale=${scaleElem.value}`;
  url += `&simd=${simdElem.checked}`;
  url += `&max_iter=${iterElem.value}`;
  url += `&min_r=${realMinElem.value}`;
  url += `&max_r=${realMaxElem.value}`;
  url += `&min_i=${imagMinElem.value}`;
  url += `&max_i=${imagMaxElem.value}`;
  url += `&red=${redElem.value}`;
  url += `&green=${greenElem.value}`;
  url += `&blue=${blueElem.value}`;
  copyLinkButton.href = url;
}

copyLinkButton.addEventListener('click', () => {
  updateLink();
  copyLinkButton.select();
});


const runWasm = async () => {
  // Instantiate our wasm module
  const wasm = await init("./pkg/web_bg.wasm");
  const { draw_mandelbrot } = wasm;




  // Listener for button click
  renderButton.addEventListener('click', () => {
    let scale = parseFloat(scaleElem.value);
    let simd = simdElem.checked;
    let max_iter = parseInt(iterElem.value);
    let width = 800 * scale;
    let height = 600 * scale;
    canvas.width = width;
    canvas.height = height;
    let minR = parseFloat(realMinElem.value);
    let maxR = parseFloat(realMaxElem.value);
    let minI = parseFloat(imagMinElem.value);
    let maxI = parseFloat(imagMaxElem.value);

    let red = parseInt(redElem.value);
    let green = parseInt(greenElem.value);
    let blue = parseInt(blueElem.value);
    let time = draw_mandelbrot(ctx, width, height, simd, max_iter, minR, maxR, minI, maxI, red, green, blue);
    time = Math.ceil(time.duration);
    outputElem.textContent = `${time}ms`;
  });

};
runWasm();