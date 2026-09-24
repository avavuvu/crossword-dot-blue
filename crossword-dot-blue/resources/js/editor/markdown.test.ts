import { expect, test } from "bun:test";
import cases from "../../../tests/markdown.json";
import { renderInline } from "./markdown";

test("renderInline matches the server", () => {
    for (const { input, output } of cases) {
        expect(renderInline(input), JSON.stringify(input)).toBe(output);
    }
});
