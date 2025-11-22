import init, { parse } from "../pkg/beta_red.js";

function handleEdit(e) {
    if (e.target.value === "") {
        output.innerText = "type something!";
        return;
    }

    try {
        let result = parse(e.target.value);
        let expr = JSON.stringify(result, null, 2);
        output.innerText = expr;
    } catch (e) {
        output.innerText = e;
    }
}

init().then(() => {
    const input = document.getElementById("input");
    const output = document.getElementById("output");

    input.addEventListener("input", handleEdit);
    input.dispatchEvent(new Event("input"));
});
