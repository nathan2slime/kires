import type { Metadata } from 'next'
import localFont from 'next/font/local'

import { cn } from '~/lib/utils'
import { AppChildren } from '~/types'

import './globals.css'

const base = localFont({ src: './fonts/ComicNeue.ttf' })

export const metadata: Metadata = {
  title: 'Kires'
}

const RootLayout = ({ children }: AppChildren) => {
  return (
    <html lang="en">
      <body className={cn('antialiased dark', base.className)}>{children}</body>
    </html>
  )
}

export default RootLayout
