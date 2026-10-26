// This release stays entirely in JS: a trapped wasm instance cannot run cleanup.
export function captureTheme(root, owner, properties) {
    const style = root.style;
    const previous = properties.map(name => [name, style.getPropertyValue(name), style.getPropertyPriority(name)]);
    const attributes = ["data-theme", "data-rustify-theme-runtime"].map(name => [name, root.getAttribute(name)]);
    return () => {
        if (root.getAttribute("data-rustify-theme-owner") !== owner) return;
        for (const [name, value, priority] of previous) {
            if (value) style.setProperty(name, value, priority);
            else style.removeProperty(name);
        }
        for (const [name, value] of attributes) {
            if (value === null) root.removeAttribute(name);
            else root.setAttribute(name, value);
        }
        root.removeAttribute("data-rustify-theme-owner");
    };
}
