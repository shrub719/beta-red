import init, { parse, evaluate, print } from "../pkg/beta_red.js";

function handleEdit(e) {
    if (e.target.value === "") {
        output.innerText = "type something!";
        return;
    }

    try {
        const parsed = parse(e.target.value + " "); // why :sob:
        const reduced = evaluate(parsed);

        const prettyReduced = print(reduced);

        output.innerText = prettyReduced;
    } catch (e) {
        output.innerHTML = "<span style='color: red'>" + e + "</span>";
    }
}

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("input", () => {
    input.style.width = "1px";
    const minWidth = 200;
    const textWidth = input.scrollWidth;
    const maxWidth = output.offsetWidth;
    console.log(maxWidth);

    if (textWidth > maxWidth) {
        input.style.width = maxWidth + "px";
    } else if (textWidth > minWidth) {
        input.style.width = textWidth + "px";
    } else {
        input.style.width = minWidth + "px";
    }
});

init().then(() => {
    input.addEventListener("input", handleEdit);
    input.dispatchEvent(new Event("input"));
});
