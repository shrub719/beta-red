import init, { upper } from "../pkg/lambda.js";

async function run() {
    await init();
}

const textInput = document.getElementById("text-input");

textInput.addEventListener("input", (e) => {
    e.target.value = upper(e.target.value);
});

run();
