const REBUS_REPLACEMENT = '\u{E000}';

export const calculateWordBoundaries = (clue: string) => {
    const rebusRegex = /(\(+.*\))/g
    const replaced = clue.replaceAll(rebusRegex, REBUS_REPLACEMENT)

    const regex = /\s/g
    const iterator = replaced.trim().matchAll(regex)
    
    return [...iterator].map(
        (match, index) => match.index - index
    )
}