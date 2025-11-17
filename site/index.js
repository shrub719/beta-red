import init, { upper } from "../pkg/lambda.js";

init().then(() => {
    const textInput = document.getElementById("text-input");

    textInput.addEventListener("input", (e) => {
        e.target.value = upper(e.target.value);
    });
});

