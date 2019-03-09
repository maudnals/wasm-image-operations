import libImgOperations from '../lib-img-operations/Cargo.toml';
import benchmark from '../utils/utils.benchmark';
import { canvasToCanvasData, drawImgOnCanvas } from '../utils/utils.canvas';
import { averageRgb } from '../utils/utils.pixels';
import { displayTextInElement } from '../utils/utils.dom';

const img = document.querySelector('#img');
const canvas = drawImgOnCanvas(img);
const arrData = canvasToCanvasData(canvas);

const arrData_int8 = new Uint8Array(arrData);
const avgWASM = benchmark(libImgOperations.avg_rgb_f64, arrData_int8);
displayTextInElement('timingJS', benchmark(averageRgb, arrData_int8));
displayTextInElement(
  'timingWASM',
  benchmark(libImgOperations.avg_rgb_f64, arrData_int8)
);

// console.log(typeof arrData);
// console.log(arrData.constructor.name);
// console.log(arrData);
// console.log(Array.from(arrData.slice(0, 8)));
// console.log(arrData.slice(0, 8));
// console.log(hasher.reduce([1, 2, 3]));
// console.log(hasher.reduce_sum_u8([1.0]));
// console.log(hasher.reduce_sum_u8(arrData));
// console.log(hasher.reduce(Array.from(arrData)));
// console.log(hasher.reduce_sum_u8(new Uint8Array(arrData)));
// const rgbs = hasher.rgbas_to_rgbs(new Uint8Array(arrData));
// console.log(rgbs);
// console.log(typeof arrData_int8);
// console.log(arrData_int8.constructor.name);
// console.log(arrData_int8);
// console.log('average, calculated with wasm', hasher.avg_rgb_f64(arrData_int8));
// console.log('average, calculated with JS', averageRgb(arrData_int8));
