export interface BooleanSetting {
    category: SettingCategory
    name: string,
    type: "bool";
    setting: boolean
}

export interface SelectSetting<T extends string = string> {
    category: SettingCategory,
    name: string;
    type: "select";
    setting: T;
    options: {
        value: T,
        name: string
    }[];
}

export type Setting = SelectSetting | BooleanSetting

export type SettingCategory = "Appearance" | "Difficulty" | "Controls"

type SettingTypes = "ArrowKeyChangeOrientation" 
    | "WordBoundaries" 
    | "NextClueSelection"
    | "Theme" 
    | "RebusWarning"
    | "ForceKeyboard"
    | "ShowTimer"
    | "ForceMobile"

export const options: Record<SettingTypes, Setting> = $state({
    "ArrowKeyChangeOrientation": {
        category: "Controls",
        name: "Arrow Keys Change Orientation",
        type: "bool",
        setting: false,
    },
    "WordBoundaries": {
        category: "Difficulty",
        name: "Enable Word Boundaries",
        type: "bool",
        setting: true,
    },
    "Theme": {
        category: "Appearance",
        name: "Theme",
        type: "select",
        setting: "default",
        options: [
            {
                value: "default",
                name: "💙 Default"
            },
            {
                value: "dark",
                name: "🌙 Dark Mode",
            },
            {
                value: "classic",
                name: "🧩Classic"
            },
            {
                value: "grove",
                name: "🪷 Grove"
            },
            {
                value: "pink",
                name: "🎀 Pink"
            },
            {
                value: "midnight",
                name: "🌚 Midnight"
            }
        ]
    },
    "NextClueSelection": {
        category: "Controls",
        name: "Select Next Clue",
        type: "select",
        setting: "ordinal",
        options: [
            {
                value: "spatial",
                name: "By Distance"
            },
            {
                value: "ordinal",
                name: "By Clue Order"
            }
        ]
    },
    "RebusWarning": {
        "category": "Difficulty",
        name: "Warn About Rebus Squares",
        type: "bool",
        setting: true,
    },
    "ForceKeyboard": {
        "category": "Controls",
        name: "Always Display Keyboard",
        type: "bool",
        setting: false
    },
    "ForceMobile": {
        "category": "Appearance",
        name: "Always Use Mobile Layout",
        "type": "bool",
        "setting": false
    },
    "ShowTimer": {
        "category": "Appearance",
        name: "Show Timer",
        type: "bool",
        setting: true
    }
    
})

export const setOptionsFromStorage = (savedOptions: Record<SettingTypes, Setting>) => {
    //check options are legit and well formed and not outdated

    Object.keys(options).forEach((key) => {
        const savedOption = savedOptions[key as SettingTypes]
        if(savedOption) {
            options[key as SettingTypes] = savedOption

        }
    })
}


const savedOptions = localStorage.getItem("options")

if(savedOptions) {
    setOptionsFromStorage(JSON.parse(savedOptions))
}