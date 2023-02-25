import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import styles from '@/styles/Home.module.css'
import { ThemeProvider, createTheme } from '@mui/material';
import { Typography, Button } from '@mui/material';
const inter = Inter({ subsets: ['latin'] })

/*
const theme = createTheme({
    palette: {
        primary: {
            main: red[500],
        },
        secondary: {
            main: '#19857b',
        }
    },
    typography: {
        myVariant: {
            fontSize: "6rem"
        }
    }
});
*/

export default function Home() {
    return (

        <>
            {/*<ThemeProvider theme={theme}>*/}
            <Head>
                <meta charSet="utf-8" />
                <link rel="apple-touch-icon" sizes="180x180" href="%PUBLIC_URL%/apple-touch-icon.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="%PUBLIC_URL%/favicon-32x32.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="%PUBLIC_URL%/favicon-16x16.png" />
                <link rel="manifest" href="%PUBLIC_URL%/site.webmanifest" />

                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="theme-color" content="#000000" />
                <meta
                    name="description"
                    content="College Guidance Reimagined"
                />
                <title>Pathfinder Guidance</title>
            </Head>
            <main className={styles.main}>
                <div className={styles.App}>
                    <Typography variant="h1">This is my app</Typography>
                    <Typography variant="h4">Sup</Typography>
                    <Button
                        color="secondary"
                        variant="contained"
                    >
                        Hello from MUI v5
                    </Button>
                </div>
            </main>
            {/*</ThemeProvider>*/}
        </>
    )
}
