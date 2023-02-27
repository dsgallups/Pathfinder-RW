"use client";

import * as React from 'react';
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
    Menu,
    MenuItem
} from '@mui/material';

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
                light: "#FFFFFF",
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
        selectedText: true;
        unselectedText: true;
        secondary: true;
    }
}


const views = ["Plans", "Grades", "Graphs"];


export default function Home() {

    const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
    const open = Boolean(anchorEl);
    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    };
    const handleClose = () => {
        setAnchorEl(null);
    };


    return (
        <ThemeProvider theme={theme}>
            <div className={styles.App}>
                <Container maxWidth="xl">
                    <Toolbar variant="regular" disableGutters sx={{
                        flexBasis: "0%"
                    }}>


                        <Box sx={{
                            display: { xs: "none", md: "flex" },
                            flex: "1 1 0%"
                        }}
                        >
                            {views.map((view) => (
                                <Button
                                    key={view}
                                    variant="contained"
                                    disableElevation
                                    color={view === "Plans" ? "selectedText" : "secondary"}
                                    disableRipple

                                    sx={{
                                        borderRadius: "10px",
                                        mr: 2,
                                        textTransform: "none",
                                        my: 0,
                                    }}
                                >{view}</Button>
                            ))}
                        </Box>

                        <Box sx={{
                            display: {
                                xs: "flex",
                                md: "flex"
                            },
                            justifyContent: "center",
                            flexDirection: "row",
                            flex: "1 1 0%"
                        }}>
                            <AcUnitIcon color="hotPink" fontSize="large" sx={{ mr: 1 }} />
                            <Typography
                                variant="h5"
                                noWrap
                                component="a"
                                href="/"
                                sx={{
                                    mt: .7,
                                    fontWeight: 700,
                                    color: "inherit",
                                    textDecoration: "none",
                                }}
                            >
                                Pathfinder
                            </Typography>
                        </Box>
                        <Box sx={{
                            display: "flex",
                            justifyContent: "flex-end",
                            flex: "1 1 0%"
                        }}>
                            <Button
                                id="basic-button"
                                variant="contained"
                                color="secondary"
                                disableElevation
                                aria-controls={open ? 'basic-menu' : undefined}
                                aria-haspopup="true"
                                aria-expanded={open ? 'true' : undefined}
                                onClick={handleClick}
                                sx={{
                                    textTransform: "none",
                                }}
                            >
                                Daniel Gallups
                            </Button>
                            <Menu
                                id="basic-menu"
                                anchorEl={anchorEl}
                                open={open}
                                onClose={handleClose}
                                MenuListProps={{
                                    'aria-labelledby': 'basic-button',
                                }}
                            >
                                <MenuItem onClick={handleClose}>Profile</MenuItem>
                                <MenuItem onClick={handleClose}>Logout</MenuItem>
                            </Menu>
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
