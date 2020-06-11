const el = document.getElementById("question-list") as HTMLUListElement;

function shuffle<T>(a: T[]): T[] {
    for (let i = a.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [a[i], a[j]] = [a[j], a[i]];
    }
    return a;
}

if (el.children[0].getAttribute("radio") !== null) {
    const radios = document.getElementsByName(el.children[0].id);
    for (const radio of radios) {
        el.children[0].appendChild(radio.parentElement!);
    }
}

function next() {
    let currentChild = el.children[0] as HTMLLIElement;
    let nextChild = el.children[1] as HTMLLIElement;

    for (let i = 1; i < el.children.length; i++) {
        if (el.children[i].getAttribute("hidden") !== null) {
            currentChild = el.children[i - 1] as HTMLLIElement;
            nextChild = el.children[i] as HTMLLIElement;
            break;
        }
    }

    let correct = false;
    if (currentChild.getAttribute("radio") !== null) {
        const radios = document.getElementsByName(currentChild.id);
        for (const radio of radios as unknown as HTMLInputElement[]) {
            if (radio.checked === true && radio.value === "1") {
                correct = true;
                break;
            }
        }
    }
    else if (currentChild.getAttribute("number") !== null) {
        let input = document.getElementById(`${currentChild.id}-n`)! as HTMLInputElement;
        let [rangeMin, rangeMax] = eval(input.getAttribute("range")!);
        if (rangeMin <= input.value && input.value < rangeMax) {
            correct = true;
        }
    }
    else if (currentChild.getAttribute("word-search") !== null) {
        correct = true;
        for (const child of currentChild.children[1].children[0].children) {
            if (child.getAttribute("finished") === null) {
                correct = false;
                break;
            }
        }
    }

    if (correct) {
        if (nextChild.getAttribute("radio") !== null) {
            const radios = document.getElementsByName(nextChild.id);
            for (const radio of radios) {
                nextChild.appendChild(radio.parentElement!);
            }
        }

        nextChild.removeAttribute("hidden");
    }
}
