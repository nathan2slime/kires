'use client'

import { invoke } from '@tauri-apps/api/core'
import { useEffect } from 'react'

import { Button } from '~/components/ui/button'

const Home = () => {
  useEffect(() => {
    onEvent()

    return () => {}
  }, [])

  const onEvent = async () => {}

  return (
    <div>
      <Button
        type="button"
        className="font-bold"
        onClick={() => {
          invoke('my_custom_command')
        }}
      >
        Invoke
      </Button>

      <span className="text-foreground">{}</span>
    </div>
  )
}
export default Home
