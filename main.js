import { canvasToCanvasData, drawImgOnCanvas } from './utils/utils.canvas';
import { averageRgb } from './utils/utils.pixels';
import hasher from './hasher/Cargo.toml';
import benchmark from './utils/utils.benchmark';
import { displayTextInElement } from './utils/utils.dom';

const img = document.querySelector('#img');
const canvas = drawImgOnCanvas(img);
const arrData = canvasToCanvasData(canvas);

console.log(typeof arrData);
console.log(arrData.constructor.name);
// console.log(arrData);
// console.log(Array.from(arrData.slice(0, 8)));
// console.log(arrData.slice(0, 8));
// console.log(hasher.reduce([1, 2, 3]));
// console.log(hasher.reduce_sum_u8([1.0]));
// console.log(hasher.reduce_sum_u8(arrData));
// console.log(hasher.reduce(Array.from(arrData)));

// gotcha 0: the setup
// gotcha 1: can not use js clamped arrays!!!
// gotcha 3: rgbas_to_rgbs: filter returns another type of data structure
// gotcha 4: panic. try for example hasher.reduce_sum_u8((arrData)) --> panic because arrData should be casted as a typed array
// console.log(hasher.reduce_sum_u8(new Uint8Array(arrData)));
// const rgbs = hasher.rgbas_to_rgbs(new Uint8Array(arrData));
// console.log(rgbs);

const arrData_int8 = new Uint8Array(arrData);
console.log(typeof arrData_int8);
console.log(arrData_int8.constructor.name);
console.log(arrData_int8);
console.log('average, calculated with wasm', hasher.avg_rgb_f64(arrData_int8));
console.log('average, calculated with JS', averageRgb(arrData_int8));

const avgWASM = benchmark(hasher.avg_rgb_f64, arrData_int8);
displayTextInElement('timingJS', benchmark(averageRgb, arrData_int8));
displayTextInElement('timingWASM', benchmark(hasher.avg_rgb_f64, arrData_int8));
