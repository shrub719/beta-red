import init, { lex } from "../pkg/lambda.js";

init().then(() => {
    const input = document.getElementById("input");
    const output = document.getElementById("output");

    input.addEventListener("input", (e) => {
        let result = lex(e.target.value);
        output.innerText = JSON.stringify(result, null, 2);
    });
});

