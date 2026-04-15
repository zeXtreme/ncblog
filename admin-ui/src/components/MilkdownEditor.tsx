import { useEffect, useRef, forwardRef, useImperativeHandle } from 'react'
import { Editor, rootCtx, defaultValueCtx } from '@milkdown/core'
import { nord } from '@milkdown/theme-nord'
import { commonmark } from '@milkdown/preset-commonmark'
import { history } from '@milkdown/plugin-history'
import { listener, listenerCtx } from '@milkdown/plugin-listener'
import '@milkdown/theme-nord/style.css'
import './MilkdownEditor.css'

export interface MilkdownEditorHandle {
  getMarkdown: () => string
}

interface Props {
  defaultValue?: string
  onChange?: (markdown: string) => void
}

// @milkdown/plugin-listener provides a listener plugin for getting markdown on change
const MilkdownEditor = forwardRef<MilkdownEditorHandle, Props>(
  ({ defaultValue = '', onChange }, ref) => {
    const containerRef = useRef<HTMLDivElement>(null)
    const markdownRef = useRef<string>(defaultValue)
    const editorRef = useRef<Editor | null>(null)

    useImperativeHandle(ref, () => ({
      getMarkdown: () => markdownRef.current,
    }))

    useEffect(() => {
      if (!containerRef.current) return

      let destroyed = false

      Editor.make()
        .config(ctx => {
          ctx.set(rootCtx, containerRef.current!)
          ctx.set(defaultValueCtx, defaultValue)
          ctx.get(listenerCtx).markdownUpdated((_ctx, markdown) => {
            markdownRef.current = markdown
            onChange?.(markdown)
          })
        })
        .config(nord)
        .use(commonmark)
        .use(history)
        .use(listener)
        .create()
        .then(editor => {
          if (!destroyed) editorRef.current = editor
        })

      return () => {
        destroyed = true
        editorRef.current?.destroy()
        editorRef.current = null
      }
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    return <div ref={containerRef} className="milkdown-wrapper" />
  }
)

MilkdownEditor.displayName = 'MilkdownEditor'
export default MilkdownEditor
