import { canvasToCanvasData, drawImgOnCanvas } from "./utils/utils.canvas";
import { averageRgb } from "./utils/utils.pixels";
import hasher from "./hasher/Cargo.toml";
import benchmark from "./utils/utils.benchmark";

const img = document.querySelector("#img");
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

const x = new Uint8Array(arrData);
console.log(typeof x);
console.log(x.constructor.name);
console.log(x);
console.log("average wasm", hasher.avg_rgb(x));
console.log("average JS", averageRgb(x));
console.log("wasm", benchmark(hasher.avg_rgb, x, 10));
console.log("JS", benchmark(averageRgb, x, 10));

// Utils
//

// var input = document.getElementById("input");
// var output = document.getElementById("output");
// output.innerText = hasher.sha1(input.value);

// input.addEventListener("keyup", function(event) {
//   output.innerText = hasher.sha1(input.value);
// });
