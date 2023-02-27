"use client";

import Image from 'next/image'
import { Inter } from 'next/font/google'
import styles from '../page.module.css'
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
import { IconPropsColorOverrides } from '@mui/material/Icon';
import { SvgIconPropsColorOverrides } from '@mui/material';
import AcUnitIcon from '@mui/icons-material/AcUnit';
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { red } from '@mui/material/colors';

const inter = Inter({ subsets: ['latin'] })

const { palette } = createTheme();

const theme = createTheme({
    palette: {
        primary: {
            main: "#F9F9FC"
        },
        secondary: {
            main: '#EFF1F5'
        },
        tertiary: palette.augmentColor({
            color: {
                light: "#9EA1AF",
                main: "#9EA1AF",
                dark: "#9EA1AF"
            }
        }),
        error: {
            main: red.A400,
        },
        hotPink: palette.augmentColor({
            color: {
                main: "#EC155B"
            }

        }),
        yellow: palette.augmentColor({
            color: {
                light: "#F5E300",
                dark: "#F5E300",
                main: "#F5E300"
            }
        }),
        green: palette.augmentColor({
            color: {
                light: "#11CB5F",
                dark: "#11CB5F",
                main: "#11CB5F"
            }
        }),
        unselectedText: palette.augmentColor({
            color: {
                light: "#9EA1AF",
                dark: "#9EA1AF",
                main: "#9EA1AF"
            }
        }),
        selectedText: palette.augmentColor({
            color: {
                light: "#000000",
                dark: "#000000",
                main: "#000000"
            }
        }),


    },
});

declare module '@mui/material/SvgIcon' {
    interface SvgIconPropsColorOverrides {
        hotPink: true;
        //color?: "hotPink" | "green" | "yellow" | "unselectedText" | "selectedText";
    }
}

declare module '@mui/material/Button' {
    interface ButtonPropsColorOverrides {
        yellow: true;
    }
}


const pages = ["About", "Features"];

export default function Home() {
    return (
        <ThemeProvider theme={theme}>
            <div className={styles.App}>
                <Container maxWidth="lg" sx={{ mt: 4 }}>
                    <Toolbar variant="regular" disableGutters>
                        {/*First break */}
                        <AcUnitIcon color="hotPink" sx={{ display: { xs: 'none', md: 'flex', fontSize: 50 }, mr: 1 }} />
                        <Typography
                            variant="h3"
                            noWrap
                            component="a"
                            href="/"
                            sx={{
                                mt: 1.5,
                                flexGrow: 1,
                                display: { xs: 'none', md: 'flex' },
                                fontWeight: 700,
                                color: "inherit",
                                textDecoration: "none",
                            }}
                        >
                            Pathfinder
                        </Typography>
                        <Box sx={{ display: { xs: "none", md: "flex" } }}>
                            {pages.map((page) => (
                                <Button
                                    key={page}
                                    disableRipple

                                    sx={{
                                        mr: 4,
                                        textTransform: "none",
                                        my: 0,
                                        color: "inherit",
                                    }}
                                >{page}</Button>
                            ))}
                        </Box>
                        <Box>
                            <Button
                                disableRipple
                                color="yellow"
                                disableElevation
                                variant="contained"
                                sx={{
                                    borderRadius: "10px",
                                    my: 0,
                                    textTransform: "none",
                                    color: "inherit",

                                }}
                            >Request Demo</Button>
                        </Box>
                    </Toolbar>
                </Container>

                <Typography variant="h1">This is my app</Typography>
                <Typography variant="h4">Sup</Typography>
                <Button
                    color="yellow"
                    variant="contained"
                >
                    Hello from MUI v5
                </Button>
            </div>
        </ThemeProvider>
    )
}
