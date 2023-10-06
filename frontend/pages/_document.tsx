import Header from '@/components/header'
import { Html, Head, Main, NextScript } from 'next/document'

export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body className='flex min-h-screen flex-col items-center justify-between bg-black'>
        <Header />
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}
