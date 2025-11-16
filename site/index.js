import * as wasm from "lambda";

const textInput = document.getElementById("text-input");

textInput.addEventListener("input", (e) => {
    e.target.value = wasm.upper(e.target.value);
});
