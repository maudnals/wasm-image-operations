import hasher from "./hasher/Cargo.toml";

var input = document.getElementById("input");
var output = document.getElementById("output");
output.innerText = hasher.sha1(input.value);

const img = document.querySelector("#img");

input.addEventListener("keyup", function(event) {
  output.innerText = hasher.sha1(input.value);
});

const canvas = document.createElement("canvas");
canvas.width = img.width;
canvas.height = img.height;
canvas.getContext("2d").drawImage(img, 0, 0, img.width, img.height);
const arrData = canvas
  .getContext("2d")
  .getImageData(0, 0, img.width, img.height).data;

console.log(arrData);
console.log(Array.from(arrData.slice(0, 8)));
console.log(arrData.slice(0, 8));

console.log(hasher.reduce([1, 2, 3]));
// console.log(hasher.reduce_u8([1.0]));
// console.log(hasher.reduce_u8(arrData));

// can not use clamped arrays!!!
console.log(hasher.reduce_u8(new Uint8Array(arrData)));

// console.log(hasher.reduce(Array.from(arrData)));
