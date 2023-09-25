export const root = () => document.body;

export const push_to = (el, html) =>
    (el.insertAdjacentHTML("beforeend", html), el.lastElementChild);

export const listen = (event, el, callback) =>
    el.addEventListener(event, callback);

export const scroll = (el) => el.scrollIntoView();

export const set_cursor = (el, position) =>
    window.getSelection().collapse(el, position);
