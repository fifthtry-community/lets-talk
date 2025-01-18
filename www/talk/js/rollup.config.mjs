import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";

export default [
    {
        input: "./index.mjs",
        output: {
            file: "./build.js",
            sourcemap: true,
            format: "iife"
        },
        plugins: [
            nodeResolve(), // Find things in node_modules (installed via npm)
            commonjs(), // Convert deps to ES modules
        ],
    },
]
