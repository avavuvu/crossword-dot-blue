import { z } from "astro:content"
import type { ZodRawShape } from "astro:schema"


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
    isRebus: z.boolean().optional().default(false),
    schema: z.string(),
    multiHintClues: z.boolean().optional().default(false),
})

export const crosswordSchema = z.object({
    metadata: metadataSchema,
    grid: z.object({
        width: z.number(),
        height: z.number(),
        cells: z.array(
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
    links: z.array(z.array(z.string())).optional(),
    clues: z.array(
        z.object({
            isHorizontal: z.boolean(),
            id: z.string(),
            hint: hintSchema,
            word: z.string(),
            coords: z.array(z.number())
        })
    )
})


export interface CwFile extends z.infer<(typeof crosswordSchema)> { };


export const extendMetadata = <T extends ZodRawShape>(metadataExtension: T) => {
    return crosswordSchema.extend({
        metadata: metadataSchema.extend(metadataExtension)
    })
}