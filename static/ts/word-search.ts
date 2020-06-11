function getRandomInt(max: number): number {
    return Math.floor(Math.random() * Math.floor(max));
}

function sum(a: number[]): number {
    return a.reduce((p, x) => p + x, 0);
}

function subVec(a: number[] | number, b: number[] | number): number[] {
    if (a instanceof Array) {
        return a.map((x, i) => {
            if (b instanceof Array) {
                return x - b[i];
            } else {
                return x - b;
            }
        });
    } else if (b instanceof Array) {
        return b.map((x) => {
            return a - x;
        });
    } else {
        return [a - b];
    }
}

function addVec(a: number[] | number, b: number[] | number): number[] {
    if (a instanceof Array) {
        return a.map((x, i) => {
            if (b instanceof Array) {
                return x + b[i];
            } else {
                return x + b;
            }
        });
    } else if (b instanceof Array) {
        return b.map((x) => {
            return a + x;
        });
    } else {
        return [a + b];
    }
}

function divVec(a: number[] | number, b: number[] | number): number[] {
    if (a instanceof Array) {
        return a.map((x, i) => {
            if (b instanceof Array) {
                return x / b[i];
            } else {
                return x / b;
            }
        });
    } else if (b instanceof Array) {
        return b.map((x) => {
            return a / x;
        });
    } else {
        return [a / b];
    }
}

function mulVec(a: number[] | number, b: number[] | number): number[] {
    if (a instanceof Array) {
        return a.map((x, i) => {
            if (b instanceof Array) {
                return x * b[i];
            } else {
                return x * b;
            }
        });
    } else if (b instanceof Array) {
        return b.map((x) => {
            return a * x;
        });
    } else {
        return [a * b];
    }
}

(() => {
    let mousePressed = false;
    document.body.addEventListener(
        "mousedown",
        (ev) => {
            if (ev.button == 0) {
                mousePressed = true;
            }
        },
    );
    document.body.addEventListener(
        "mouseup",
        (ev) => {
            if (ev.button == 0) {
                mousePressed = false;
            }
        },
    );

    const games = document.querySelectorAll<HTMLDivElement>("div.word-search");

    const letters = "abcdefghijklmnopqrstuvwxyz";
    let gameTargets: {
        selectedData: { pos: number[]; dir: number[]; len: number; };
        shape: number[];
    }[] = [];

    function createCharElement(
        char: string,
        pos: number[],
        gameIdx: number,
    ): HTMLElement {
        const target = gameTargets[gameIdx];
        const game = games[gameIdx];
        const newEl = document.createElement("button");

        newEl.style.padding = "0px";
        newEl.style.border = "none";
        newEl.style.backgroundColor = "transparent";
        newEl.style.outline = "none";
        newEl.setAttribute("Highlight", "false");

        newEl.appendChild(document.createTextNode(char));

        const clear = () => {
            for (
                const butt of game.children[1]
                    .children as unknown as HTMLButtonElement[]
            ) {
                butt.setAttribute("Highlight", "false");
            }
        };

        const over = () => {
            if (mousePressed) {
                clear();
                let dir: number[] = [];
                if (
                    pos[0] == target.selectedData.pos[0] &&
                    0 <= pos[1] - target.selectedData.pos[1]
                ) {
                    dir = [0, 1];
                } else if (
                    0 <= pos[0] - target.selectedData.pos[0] &&
                    pos[1] == target.selectedData.pos[1]
                ) {
                    dir = [1, 0];
                } else if (
                    0 <= pos[0] - target.selectedData.pos[0] &&
                    sum(mulVec(subVec(pos, target.selectedData.pos), [1, -1])) === 0
                ) {
                    dir = [1, 1];
                } else {
                    return;
                }

                const len = (sum(pos) - sum(target.selectedData.pos)) / sum(dir) + 1;
                let max = -1;
                for (
                    const word of game.children[0].children as unknown as HTMLLIElement[]
                ) {
                    const len = word.innerText.replace(" ", "").length;
                    if (max < len) {
                        max = len;
                    }
                }
                for (
                    const word of game.children[0].children as unknown as HTMLLIElement[]
                ) {
                    if (word.getAttribute("finished") !== null) {
                        continue;
                    }
                    const word_txt = word.innerText.toLowerCase().replace(" ", "");
                    let p = target.selectedData.pos;
                    let good = true;
                    for (const ch of word_txt.split("").slice(0, len)) {
                        let currentEl = game.children[1]
                            .children[p[1] + p[0] * target.shape[1]] as HTMLButtonElement;

                        if (ch === currentEl.innerText) {
                            currentEl.setAttribute("highlight", "true");
                            p = addVec(p, dir);
                        } else {
                            good = false;
                            break;
                        }
                    }
                    if (len === word_txt.length && good) {
                        word.setAttribute("finished", "");
                        p = target.selectedData.pos;
                        for (const ch of word_txt.split("").slice(0, len)) {
                            let currentEl = game.children[1]
                                .children[p[1] + p[0] * target.shape[1]] as HTMLButtonElement;
                            currentEl.setAttribute("finished", "");

                            p = addVec(p, dir);
                        }
                        break;
                    }
                }

                target.selectedData.dir = dir;
            }
        };

        const down = () => {
            target.selectedData.pos = pos;
            mousePressed = true;
            over();
        };

        newEl.addEventListener("mousedown", down);
        newEl.addEventListener("mouseup", clear);
        newEl.addEventListener("mouseover", over);

        newEl.addEventListener("touchstart", down);
        newEl.addEventListener("touchend", clear);
        // newEl.addEventListener("touchmove", over);

        return newEl;
    }

    for (const [gameIdx, game] of games.entries()) {
        let word_search_table: string[][] = [];
        const shape: number[] = eval(game.getAttribute("shape")!);
        gameTargets.push(
            { selectedData: { pos: [0, 0], dir: [0, 0], len: 0 }, shape },
        );

        for (let i = 0; i < shape[0]; i++) {
            word_search_table.push([]);
            for (let j = 0; j < shape[1]; j++) {
                word_search_table[i].push(" ");
            }
        }

        for (
            const word of game.children[0].children as unknown as HTMLLIElement[]
        ) {
            const word_txt = word.innerText.toLowerCase().replace(" ", "");
            while (true) {
                let pos = [0, 0];
                let dir = [0, 0];
                switch (getRandomInt(3)) {
                    case 0: // Horizontal
                        pos = [
                            getRandomInt(shape[0] - word_txt.length),
                            getRandomInt(shape[1]),
                        ];
                        dir = [1, 0];
                        break;
                    case 1: // Vertical
                        pos = [
                            getRandomInt(shape[0]),
                            getRandomInt(shape[1] - word_txt.length),
                        ];
                        dir = [0, 1];
                        break;
                    case 2: // Diagonal
                        pos = [
                            getRandomInt(shape[0] - word_txt.length),
                            getRandomInt(shape[1] - word_txt.length),
                        ];
                        dir = [1, 1];
                        break;
                }
                let done = true;
                for (
                    let i = 0;
                    i < word_txt.length;
                    i++, pos = addVec(pos, dir)
                ) {
                    const prev = word_search_table[pos[0]][pos[1]];
                    if (prev !== " " && prev !== word_txt[i]) {
                        done = false;
                        break;
                    }
                }
                if (!done) continue;

                pos = subVec(pos, mulVec(word_txt.length, dir));

                for (
                    let i = 0;
                    i < word_txt.length;
                    i++, pos = addVec(pos, dir)
                ) {
                    word_search_table[pos[0]][pos[1]] = word_txt[i];
                }

                break;
            }
        }

        const grid = game.children[1] as HTMLDivElement;
        let touchEl: HTMLElement | null = null;
        grid.addEventListener("touchmove", (ev) => {
            let el = document.elementFromPoint(
                ev.touches[0].clientX,
                ev.touches[0].clientY,
            )! as HTMLElement;
            if (touchEl !== el) {
                let newEv = new MouseEvent("mouseover");
                el.dispatchEvent(newEv);
                touchEl = el;
            }
        });
        grid.addEventListener("touchend", () => touchEl = null);

        for (let i = 0; i < shape[0]; i++) {
            for (let j = 0; j < shape[1]; j++) {
                const ch: string = (() => {
                    if (word_search_table[i][j] === " ") {
                        return letters[getRandomInt(letters.length)];
                    } else {
                        return word_search_table[i][j];
                    }
                })();
                grid.appendChild(createCharElement(ch, [i, j], gameIdx));
            }
        }

        grid.style.gridTemplateColumns = `repeat(${shape[0]}, 20px)`;
    }
})();
