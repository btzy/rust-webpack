export function compile(code) {
    return import("../pkg/index.js").then(module => module.compile(code));
}