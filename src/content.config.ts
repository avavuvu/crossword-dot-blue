import type { CrosswordCollection } from '$lib/game/types';
import { file, glob } from 'astro/loaders';
import { defineCollection, z } from 'astro:content';
import fs from "node:fs"
import { XMLParser } from "fast-xml-parser"
import { crosswordSchema, extendMetadata } from '$lib/cw/cw';
import { crosswordLoader } from '$lib/cw/loader';

const blueSchema = extendMetadata({
    name: z.string().optional(),
    id: z.string(),
    difficulty: z.number().gt(0).lte(5),
})

export interface CwFile extends z.infer<(typeof blueSchema)> { }
export type CrosswordMetadata = CwFile["metadata"] & { collection: CrosswordCollection }


const big = defineCollection({
    loader: crosswordLoader("big"),
    schema: blueSchema
})

const mini = defineCollection({
    loader: crosswordLoader("mini"),
    schema: blueSchema
})

export const collections = {
    mini, big
}
