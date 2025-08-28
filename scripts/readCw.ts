import { boolean, z } from "zod";
import { XMLParser } from "fast-xml-parser";
import fs from "node:fs"

const hintSchema = z.object({
        text: z.string(),
        display: z.union([
            z.literal("markdown"),
            z.literal("plaintext")
        ]).optional()
    })

const metadataSchema = z.object({
            author: z.string(),
            date: z.coerce.date(),
            isRebus: z.boolean().default(false),
            schema: z.string(),
            multiHintClues: z.boolean().default(false),
        }).loose()

const schema = z.object({
        metadata: metadataSchema,
        grid: z.object({
            width: z.number(),
            height: z.number(),
            cell: z.array(
                z.object({
                    type: z.literal("text"),
                    text: z.string(),
                    style: z.object({
                        circled: z.boolean().optional(),
                        gray: z.boolean().optional()
                    }).optional()
                }).or(z.object({
                    type: z.literal("block")
                }))
            ),
        }),
        links: z.array(z.object({
            group: z.array(z.object({
                id: z.string()
            }))
        })).optional(),
        clues: z.array(
            z.object({
                isHorizontal: z.boolean(),
                id: z.string(),
                hint: z.array(hintSchema).or(hintSchema),
                word: z.string(),
                coords: z.array(z.number())
            })
        )
    })

const extendMetadata = (metadataExtension: Record<string, any>) => {
    return schema.extend({
        metadata: metadataSchema.extend(metadataExtension)
    })
}

const blueSchema = extendMetadata({
    name: z.string().optional(),
    id: z.string(),
    difficulty: z.number().gt(0).lte(5),
})



interface Crossword {
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
  links?: Array<{
    group: Array<{
      id: string;
    }>;
  }>;
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


const loadCrossword = async () => {
    const files = fs.readdirSync("./src/collections/mini");

    const parser = new XMLParser({
        "ignoreAttributes": false,
        "attributeNamePrefix": "",
        "parseAttributeValue": true,
        "textNodeName": "@text",
        isArray(tagName, jPath, isLeafNode, isAttribute) {
            if(tagName === "hint") { return true}

            return false
        },
    })


    const a = files
        .filter(file => file.endsWith(".cw"))
        .map(file => {
            const buffer = fs.readFileSync(`./src/collections/mini/${file}`)
            const data = buffer.toString()

            const { crossword }: {
                crossword: Crossword
            } = parser.parse(data)

            const grid = { 
                ...crossword.grid,
                cell: crossword.grid.cell.map(cell => {
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
            }

            console.log(crossword.clues)

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
                })
            }))


            return {
                id: file,
                ...crossword,
                clues, grid
            }
        })

    return a[0]
}

const cwFile = await loadCrossword()
const parsed = blueSchema.safeParse(cwFile)

if(parsed.error) {
    console.log(z.prettifyError(parsed.error))
}else {
    console.log(parsed.success)
    console.log(parsed.data.grid.cell[0])
}
