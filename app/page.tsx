'use client'

import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

import { Button } from '~/components/ui/button'
import { AnimeData } from '~/types/anime'

const Home = () => {
  const [anime, setAnime] = useState<AnimeData>();

  useEffect(() => {
    invoke('scan_anime')

    onEvent()

    return () => { }
  }, [])

  const onEvent = async () => {
    listen<AnimeData>('anime_detected', event => {
      setAnime(event.payload)
    })
  }

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

      <span className='text-foreground'>
        {anime && JSON.stringify(anime)}
      </span>
    </div>
  )
}
export default Home
