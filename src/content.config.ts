import { glob } from 'astro/loaders';
import { defineCollection, z } from 'astro:content';

const crosswordSchema = z.object({
    metadata: z.object({
        author: z.string(),
        date: z.coerce.date(),
        name: z.string(),
        difficulty: z.number().gt(0).lte(5),
        rebus: z.boolean().optional(),
        characterSet: z.union([
            z.literal("default"),
            z.literal("numbers"),
            z.literal("special"),
        ]).optional()
    }),
    content: z.object({
        height: z.number().gte(4),
        width: z.number().gte(4),
        grid: z.array(
            z.string()
        ),
        links: z.array(
            z.array(z.string())
        ).optional(),
        clues: z.array(
            z.object({
                clue: z.string(),
                coords: z.array(z.number()),
                id: z.string(),
                isHorizontal: z.boolean(),
                word: z.string(),
                wordBoundaries: z.array(z.number()).optional()
            })
        )
    })
})

const big = defineCollection({ 
    loader: glob({ 
        pattern: "**/*.json", 
        base: "./src/collections/big" 
    }),
    schema: crosswordSchema
})

const mini = defineCollection({ 
    loader: glob({ 
        pattern: "**/*.json", 
        base: "./src/collections/mini" 
    }),
    schema: crosswordSchema
})

export const collections = { 
    big, mini
}
