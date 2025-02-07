export interface PuzzleShareData {
    time: string,
    url: string
}

export const share = async (navigator: Navigator, {time, url}: PuzzleShareData): Promise<boolean> => {
    const shareData = {
        title: "Crossword Dot Blue",
        text: `I solved this puzzle in ${time} seconds! Play it now at ${url}`
    }
    
    if(navigator.share !== undefined) {
        await navigator.share(shareData)
        return true
    }

    navigator.clipboard.writeText(shareData.text)
    return false
}