/*
DOM commands (side effects)
*/

export const displayTextInElement = (id, text) => {
  document.getElementById(id).innerText = text;
};
