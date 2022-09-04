// Import React dependencies.
import {useState} from 'react'
// Import the Slate editor factory.
import {BaseEditor, Descendant, createEditor} from 'slate'
// Import the Slate components and React plugin.
import {Editable, Slate, withReact} from 'slate-react'
import {ReactEditor} from 'slate-react'

type CustomElement = {type: 'paragraph'; children: CustomText[]}
type CustomText = {text: string}

declare module 'slate' {
    interface CustomTypes {
        Editor: BaseEditor & ReactEditor
        Element: CustomElement
        Text: CustomText
    }
}
export const InputComponent = () => {
    const [editor] = useState(() => withReact(createEditor()))
    const initValues: Descendant[] = []
    const onChange = (value: Descendant[]) => {
        console.log('onchange value', value)
    }

    return (
        <Slate editor={editor} value={initValues} onChange={onChange}>
            <Editable autoFocus></Editable>
        </Slate>
    )
}
