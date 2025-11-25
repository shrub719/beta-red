import init, { parse, evaluate, print } from "../pkg/beta_red.js";

function handleEdit(e) {
    if (e.target.value === "") {
        output.innerText = "type something!";
        return;
    }

    try {
        let parsed = parse(e.target.value + " "); // why :sob:
        let reduced = evaluate(result);

        let prettyParsed = print(result);
        let prettyReduced = print(reduced);

        output.innerText = prettyParsed + "\n" + prettyReduced;
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
