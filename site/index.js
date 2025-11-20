import init, { parse } from "../pkg/lambda.js";

init().then(() => {
    const input = document.getElementById("input");
    const output = document.getElementById("output");

    input.addEventListener("input", (e) => {
        try {
            let result = parse(e.target.value);
            output.innerText = JSON.stringify(result, null, 2);
        } catch (e) {
            console.error(e);
        }
    });
});

