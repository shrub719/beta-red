import init, { parse, evaluate, print } from "../pkg/beta_red.js";

function handleEdit(e) {
    if (e.target.value === "") {
        output.innerText = "type something!";
        return;
    }

    try {
        let parsed = parse(e.target.value + " "); // why :sob:
        let reduced = evaluate(parsed);

        let prettyReduced = print(reduced);

        output.innerText = prettyReduced;
    } catch (e) {
        output.innerText = e;
    }
}

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("input", () => {
    input.style.width = "1px";
    let scrollWidth = input.scrollWidth;
    console.log(scrollWidth);
    if (scrollWidth > 200) {
        input.style.width = input.scrollWidth + "px";
    } else {
        input.style.width = "200px";
    }
});

init().then(() => {
    input.addEventListener("input", handleEdit);
    input.dispatchEvent(new Event("input"));
});
