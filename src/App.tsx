import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { checkGrammar } from './services/chatgpt'
import { GrammarFeedback } from './components/GrammarFeedback'
import { register } from '@tauri-apps/plugin-global-shortcut'
import { readText } from '@tauri-apps/plugin-clipboard-manager'
import { listen } from '@tauri-apps/api/event'
import { Command } from '@tauri-apps/plugin-shell'

import './App.css'

// https://github.com/tauri-apps/tauri/discussions/5624

const script = `
tell application "System Events"
    keystroke "c" using {command down}
end tell
`

const args = script
  .trim()
  .split('\n')
  .flatMap((line) => ['-e', line])

// const command = new Command("osascript", args);
// const command = Command.create("osascript", args);
console.log('args', args)

const command = Command.create('run-osascript', args)

async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

async function getSelectedText() {
  const initialClipboardContent = await readText()

  // try {
  await command.spawn()

  await new Promise((resolve) => {
    command.on('close', resolve)
  })

  let attempts = 10
  let currentClipboardContent = initialClipboardContent

  while (attempts > 0 && currentClipboardContent === initialClipboardContent) {
    await sleep(100)
    currentClipboardContent = await readText()
    attempts--
  }
  console.log('currentClipboardContent', currentClipboardContent)

  return currentClipboardContent
  // } finally {
  //   if (initialClipboardContent) {
  //     await writeText(initialClipboardContent);
  //   }
  // }
}

async function getSelectionText(): Promise<String> {
  return invoke<String>('get_selection_text')
}

function App() {
  const [feedback, setFeedback] = useState<string | null>(null)
  const [isLoading, setIsLoading] = useState(false)

  const [selectedText, setSelectedText] = useState<string | null>(null)

  useEffect(() => {
    // const isRegistered = await isRegistered('CommandOrControl+P');
    // const fn = async () => {
    //   await register('Command+Z', (event) => {
    //     if (event.state === "Pressed") {
    //         console.log('Shortcut triggered');
    //     }
    //   });
    // };

    // fn()

    listen('shortcut-event', async (event) => {
      console.log('shortcut-event 前端监听到了')
      const text = (await getSelectionText()).toString()
      setSelectedText(text)
      handleGrammarCheck(text)

      // const text = await getSelectedText();

      console.log('text', text)

      console.log(event.payload) // 打印事件的负载，例如 "Ctrl+D triggered"
    })

    // const handleKeyDown = async (event: KeyboardEvent) => {
    //   // Check if Ctrl+Z (Windows) or Cmd+Z (Mac) is pressed
    //   if ((event.ctrlKey || event.metaKey) && event.key === 'z') {
    //     event.preventDefault() // Prevent the default undo action
    //     await handleGrammarCheck()
    //   }
    // }

    async function handleGrammarCheck(selectedText: string) {
      try {
        setIsLoading(true)
        // const selectedText = await invoke<string>('get_selected_text')

        // console.log
        // if (selectedText) {
        //   const grammarFeedback = await checkGrammar(selectedText);
        //   setFeedback(grammarFeedback);
        // }
        const grammarFeedback = await checkGrammar(selectedText)
        setFeedback(grammarFeedback)
      } catch (error) {
        console.error('Error:', error)
        setFeedback('Error checking grammar. Please try again.')
      } finally {
        setIsLoading(false)
      }
    }

    // Add event listener
    // window.addEventListener('keydown', handleKeyDown)

    // // Cleanup
    // return () => {
    //   window.removeEventListener('keydown', handleKeyDown)
    // }
  }, [])

  return (
    <div className="p-4">
      <div className="text-2xl font-bold text-center">英语语法检查</div>
      <p className="text-center mt-4">
        选择任意文本，按 Ctrl+D (Cmd+D on Mac) 检查语法
      </p>

      <div className="mt-10 text-center bg-gray-200 p-4">{selectedText}</div>
      <GrammarFeedback feedback={feedback} isLoading={isLoading} />
    </div>
  )
}

export default App
