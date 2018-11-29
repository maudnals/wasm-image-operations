import hasher from "./hasher/Cargo.toml";

var input = document.getElementById("input");
var output = document.getElementById("output");
output.innerText = hasher.sha1(input.value);
// Utils

const canvasToCanvasData = canvas =>
  canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height).data;

const img = document.querySelector("#img");

input.addEventListener("keyup", function(event) {
  output.innerText = hasher.sha1(input.value);
});

const canvas = document.createElement("canvas");
canvas.width = img.width;
canvas.height = img.height;
canvas.getContext("2d").drawImage(img, 0, 0, img.width, img.height);
const arrData = canvasToCanvasData(canvas);

console.log(arrData);
console.log(Array.from(arrData.slice(0, 8)));
console.log(arrData.slice(0, 8));

console.log(hasher.reduce([1, 2, 3]));
// console.log(hasher.reduce_sum_u8([1.0]));
// console.log(hasher.reduce_sum_u8(arrData));

// gotcha 0: the setup
// gotcha 1: can not use js clamped arrays!!!
// gotcha 3: rgbas_to_rgbs: filter returns another type of data structure
// gotcha 4: panic. try for example hasher.reduce_sum_u8((arrData)) --> panic because arrData should be casted as typed array
console.log(hasher.reduce_sum_u8(new Uint8Array(arrData)));
const rgbs = hasher.rgbas_to_rgbs(new Uint8Array(arrData));
console.log(rgbs);
console.log("average", hasher.avg(new Uint8Array(arrData)));

// console.log(hasher.reduce(Array.from(arrData)));

// Utils

// const canvasToCanvasData = canvas =>
//   canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height).data;
