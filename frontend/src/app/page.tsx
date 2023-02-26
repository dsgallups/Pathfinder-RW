"use client";

import Image from 'next/image'
import { Inter } from 'next/font/google'
import styles from './page.module.css'
import { Typography, Button } from '@mui/material';
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { red } from '@mui/material/colors';

const inter = Inter({ subsets: ['latin'] })


const theme = createTheme({
    palette: {
        primary: {
            main: red[500],
        },
        secondary: {
            main: '#19857b',
        },
        error: {
            main: red.A400,
        },
    },
});

export default function Home() {
    return (
        <ThemeProvider theme={theme}>
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
        </ThemeProvider>
    )
}
