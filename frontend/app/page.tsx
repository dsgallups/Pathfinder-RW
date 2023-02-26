"use client";

import Image from 'next/image'
import { Inter } from 'next/font/google'
import styles from './page.module.css'
import {
    Typography,
    Button,
    AppBar,
    Toolbar,
    IconButton,
    Container,
    Box,
} from '@mui/material';
import MenuIcon from "@mui/icons-material/Menu";
import AdbIcon from '@mui/icons-material/Adb';
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { red } from '@mui/material/colors';

const inter = Inter({ subsets: ['latin'] })


const theme = createTheme({
    palette: {
        primary: {
            main: red[500],
        },
        secondary: {
            main: '#11cb5f',
        },
        error: {
            main: red.A400,
        },
    },
});

const pages = ["Home", "About", "Contact", "Blog"];

export default function Home() {
    return (
        <ThemeProvider theme={theme}>
            <div className={styles.App}>
                <AppBar position="static" color="secondary">
                    <Container maxWidth="lg">
                        <Toolbar variant="regular" disableGutters>
                            {/*First break */}
                            <AdbIcon sx={{ display: { xs: 'none', md: 'flex' }, mr: 1 }} />
                            <Typography
                                variant="h6"
                                noWrap
                                component="a"
                                href="/"
                                sx={{
                                    mr: 2,
                                    display: { xs: 'none', md: 'flex' },
                                    fontWeight: 700,
                                    color: "inherit",
                                    textDecoration: "none"
                                }}
                            >
                                Pathfinder Guidance
                            </Typography>
                            <Box sx={{ flexGrow: 1, display: { xs: "none", md: "flex" } }}>
                                {pages.map((page) => (
                                    <Button
                                        key={page}
                                        sx={{
                                            my: 2,
                                            color: "inherit",
                                        }}
                                    >{page}</Button>
                                ))}
                            </Box>
                            <IconButton edge="start" color="inherit" aria-label="menu" sx={{ mr: 2 }}>
                                <MenuIcon />
                            </IconButton>
                            <Typography variant="h6" color="inherit" component="div">
                                Photos
                            </Typography>
                        </Toolbar>
                    </Container>

                </AppBar>
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
