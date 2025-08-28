import type { Loader } from "astro/loaders";
import { XMLParser } from "fast-xml-parser";
import fs from "node:fs"
import { crosswordSchema } from "./cw";
import type { z } from "astro:content";

// TODO: switch to a more reliable xml parser

interface CwStructure {
  metadata: {
    author: string;
    date: Date;
    name?: string;
    isRebus?: boolean;
  };
  grid: {
    width: number;
    height: number;
    cell: Array<
      | {
          text: string;
          style?: {
            circled?: boolean;
            gray?: boolean;
            
          } & Record<string, any>;
        }
      | {
          block: "";
        }
    >;
  };
  links?: {
        group: {
            ids: string;
        }[]
    };
  clues: {clue: Array<{
    isHorizontal: boolean;
    id: string;
    hint: { "@text": string, display?: string}[] | string;
    word: string;
    coords: {
      c: number[];
    };
  }>}
}

interface CrosswordFile extends z.infer<(typeof crosswordSchema)> { }

export const crosswordLoader = (folder: string): Loader => ({
    schema: crosswordSchema,
    name: "crossword-loader",
    load: async (context) => {
        const files = fs.readdirSync(`./src/collections/${folder}`);

        const parser = new XMLParser({
            "ignoreAttributes": false,
            "attributeNamePrefix": "",
            "parseAttributeValue": true,
            "textNodeName": "@text",
            isArray(tagName, jPath, isLeafNode, isAttribute) {
                if(["hint", "group"].includes(tagName)) { return true}

                return false
            },
        })


        const a = files
            .filter(file => file.endsWith(".cw"))
            .map(file => {
                console.log("reading file", file)
                const buffer = fs.readFileSync(`./src/collections/${folder}/${file}`)
                const data = buffer.toString()

                const { crossword }: {
                    crossword: CwStructure
                } = parser.parse(data)

                const grid = { 
                    width: crossword.grid.width,
                    height: crossword.grid.height,
                    cells: crossword.grid.cell.map(cell => {
                        if("text" in cell) {
                            return {
                                type: "text",
                                text: cell.text,
                                style: cell.style
                            }
                        }

                        return {
                            type: "block"
                        }
                    })
                } as CrosswordFile["grid"]

                const clues = crossword.clues.clue.map((clue: any) => ({
                    ...clue,
                    coords: clue.coords.split(" ").map((index: string) => parseInt(index)),
                    hint: clue.hint.map((hint: string | { "@text": string, display: string}) => {
                        if(typeof hint === "string") {
                            return {
                                text: hint
                            }
                        }

                        return {
                            text: hint["@text"],
                            display: hint.display
                        }
                    })[0]
                })) as CrosswordFile["clues"]

                console.log(crossword.metadata)

                const metadata = {
                    ...crossword.metadata,
                    date: new Date(crossword.metadata.date)
                } as CrosswordFile["metadata"]

                console.log(crossword.links, file)

                const links = (crossword.links?.group ?? [])
                    .map(link => link.ids.split(" ")
                    ) as NonNullable<CrosswordFile["links"]>

                return {
                    id: file,
                    ...crossword,
                    metadata,
                    clues, grid, links
                }
            })

        for(const crossword of a) {
            context.parseData({
                id: crossword.id,
                data: crossword
            })

            context.store.set({
                id: crossword.id,
                data: crossword,
            });
        }

    }
})