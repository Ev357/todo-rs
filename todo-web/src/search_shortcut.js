window.addEventListener("keydown", (event) => {
    if (
        event.key === "/" &&
        !["INPUT", "TEXTAREA"].includes(document.activeElement?.tagName)
    ) {
        event.preventDefault();
        document.querySelector('input[name="search"]')?.focus();
    }
});
