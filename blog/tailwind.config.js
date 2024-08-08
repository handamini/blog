import typography from "@tailwindcss/typography";
import daisyui from "daisyui";

export const content = {
    files: ["*.html", "./src/**/*.rs"],
};
export const theme = {
    extend: {},
};
export const plugins = [typography, daisyui];
export const daisyui = {};
