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
    MenuItem,
    Avatar,
    Grid,
    Fab,
    Chip
} from '@mui/material';

import { IconPropsColorOverrides } from '@mui/material/Icon';
import { SvgIconPropsColorOverrides } from '@mui/material';
import AcUnitIcon from '@mui/icons-material/AcUnit';
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { deepOrange, red } from '@mui/material/colors';
import UnfoldMoreIcon from '@mui/icons-material/UnfoldMore';
import Grid2 from '@mui/material/Unstable_Grid2/Grid2';
import ButtonBase from '@mui/material/ButtonBase';
import Brightness1Icon from '@mui/icons-material/Brightness1';

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
        green: true;
        yellow: true;
        //color?: "hotPink" | "green" | "yellow" | "unselectedText" | "selectedText";
    }
}

declare module '@mui/material/Button' {
    interface ButtonPropsColorOverrides {
        yellow: true;
        selectedText: true;
        unselectedText: true;
        secondary: true;
        green: true;
    }
}
declare module '@mui/material/Fab' {
    interface FabPropsColorOverrides {
        green: true;
        yellow: true;
    }
}
declare module '@mui/material/Chip' {
    interface ChipPropsColorOverrides {
        green: true;
        yellow: true;
        selectedText: true;
        unselectedText: true;
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
                                    borderRadius: "10px"

                                }}
                            >
                                <Avatar sx={{
                                    width: 18,
                                    height: 18,
                                    bgcolor: deepOrange[500],
                                    mr: 1,
                                    mb: .5
                                }}>DG</Avatar>
                                <Typography
                                    variant="subtitle1"
                                    sx={{
                                        fontWeight: 500
                                    }}
                                >
                                    Daniel Gallups
                                </Typography>

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
                <Grid2
                    container
                    sx={{
                        '--Grid-borderWidth': '1px',
                        borderTop: 'var(--Grid-borderWidth) solid',
                        borderLeft: 'var(--Grid-borderWidth) solid',
                        borderColor: 'divider',
                        '& > div': {
                            borderRight: 'var(--Grid-borderWidth) solid',
                            borderBottom: 'var(--Grid-borderWidth) solid',
                            borderColor: 'divider',
                        },
                    }}
                >
                    <Grid2

                        xs
                        sx={{
                            display: "flex",
                            flexDirection: "row",
                            alignItems: "flex-start",
                            backgroundColor: "primary.main",
                            justifyContent: "space-between",
                            p: 2,

                        }}>
                        <Grid2 container sx={{
                            display: "flex",
                            flexDirection: "column",
                            alignItems: "flex-start",
                        }}>

                            <Typography sx={{ fontWeight: "700" }}>Current Plan</Typography>
                            <Typography>Fall 2020 - Fall 2022</Typography>



                        </Grid2>
                        <UnfoldMoreIcon sx={{ alignSelf: "center" }} />
                    </Grid2>


                    <Grid2 xs sx={{
                        display: "flex",
                        flexDirection: "row",
                        alignItems: "flex-start",
                        backgroundColor: "primary.main",
                        p: 2,
                    }}>
                        <Chip
                            label="Major"
                            color="green"
                            size="small"
                            sx={{
                                fontWeight: "700",
                                fontSize: ".8rem",
                                mr: 2,
                                color: "white",
                            }}
                        />
                        <Grid2 container sx={{
                            display: "flex",
                            flexDirection: "column",
                            alignItems: "flex-start",
                        }}>
                            <Typography sx={{ fontWeight: "700" }}>Cybersecurity</Typography>
                            <Typography variant="subtitle2" sx={{ color: "unselectedText.main" }}>& Marketing</Typography>
                        </Grid2>
                    </Grid2>

                    <Grid2 xs sx={{
                        display: "flex",
                        flexDirection: "row",
                        alignItems: "flex-start",
                        backgroundColor: "primary.main",
                        p: 2,
                    }}>
                        <Chip
                            label="Minor"
                            color="yellow"
                            size="small"
                            sx={{
                                fontWeight: "700",
                                fontSize: ".8rem",
                                mr: 2,
                            }}
                        />
                        <Grid2 container sx={{
                            display: "flex",
                            flexDirection: "column",
                            alignItems: "flex-start",
                        }}>
                            <Typography sx={{ fontWeight: "700" }}>Entreprenership</Typography>
                            <Typography noWrap variant="subtitle2" sx={{ color: "unselectedText.main" }}>Data Science, Forens...</Typography>
                        </Grid2>
                    </Grid2>

                    <Grid2 xs sx={{
                        display: "flex",
                        flexDirection: "row",
                        alignItems: "flex-start",
                        backgroundColor: "primary.main",
                        p: 2,
                    }}>
                        <Chip
                            label="Credits"
                            variant="outlined"
                            color="selectedText"
                            size="small"
                            sx={{
                                fontWeight: "700",
                                fontSize: ".7rem",
                                mr: 2,
                            }}
                        />
                        <Grid2 container sx={{
                            display: "flex",
                            flexDirection: "column",
                            alignItems: "flex-start",
                        }}>
                            <Typography sx={{ fontWeight: "700" }}>132 | 47</Typography>
                            <Typography noWrap variant="subtitle2" sx={{ color: "unselectedText.main" }}>126 remaining</Typography>
                        </Grid2>
                    </Grid2>

                    <Grid2 xs sx={{
                        display: "flex",
                        flexDirection: "row",
                        alignItems: "flex-start",
                        backgroundColor: "primary.main",
                        p: 2,
                    }}>
                        <Chip
                            label="GPA"
                            variant="outlined"
                            color="selectedText"
                            size="small"
                            sx={{
                                fontWeight: "700",
                                fontSize: ".7rem",
                                mr: 2,
                            }}
                        />
                        <Grid2 container sx={{
                            display: "flex",
                            flexDirection: "column",
                            alignItems: "flex-start",
                        }}>
                            <Typography sx={{ fontWeight: "700" }}>3.62 / 4.00</Typography>
                            <Typography noWrap variant="subtitle2" sx={{ color: "unselectedText.main" }}>.02 (9/12/20)</Typography>
                        </Grid2>
                    </Grid2>
                </Grid2>

                <Grid2 container sx={{
                    backgroundColor: "primary.main",
                }}>
                    {/*xs={12} md={6} lg={2} */}
                    <Grid2 xs={12} md={6} lg={2}>
                        <Grid2 container sx={{
                            display: "flex",
                            flexDirection: "column",
                            maxWidth: "300px"
                        }}>
                            <Grid2 sx={{
                                display: "flex",
                                flexDirection: "row",
                                alignItems: "flex-end",
                                justifyContent: "space-between",
                                m: 2,
                            }}>
                                <Typography sx={{ fontWeight: "700", alignSelf: "flex-start" }}>Fall 2020</Typography>
                                <Grid2 sx={{
                                    display: "flex",
                                    flexDirection: "row",
                                    alignItems: "flex-end"
                                }}>
                                    {/*variant="outlined"
                                        color="selectedText"
                                        disableElevation
                                        size="small"
                                        sx={{
                                            textTransform: "none",
                                            fontWeight: "700",
                                            fontSize: ".7rem",
                                            borderRadius: "25px",
                                            alignItems: "center",
                                            mb: 0,
                                            pt: .5,
                                            pb: .2,
                                            pr: 0,
                                            pl: 0,
                                        }}*/}
                                    <Chip
                                        label="17"
                                        variant="outlined"
                                        color="selectedText"
                                        size="small"
                                        sx={{
                                            fontWeight: "700",
                                            fontSize: ".7rem",
                                        }}
                                    />
                                    <Chip
                                        label="3.23 GPA"
                                        variant="outlined"
                                        color="selectedText"
                                        size="small"
                                        sx={{
                                            fontWeight: "700",
                                            fontSize: ".7rem",
                                        }}
                                    />

                                </Grid2>
                            </Grid2>

                            <Grid2 container sx={{
                                backgroundColor: "secondary.main",
                                borderRadius: "10px",
                            }}>
                                <Grid2 container sx={{
                                    display: "flex",
                                    flexDirection: "row",
                                    justifyContent: "space-between",
                                    backgroundColor: "#FFF",
                                    p: 1.5,
                                    m: 1.5,
                                    flexGrow: 1,
                                    borderRadius: "6px",
                                }}>

                                    <Grid2 container sx={{
                                        display: "flex",
                                        alignItems: "center",
                                    }}>

                                        <Brightness1Icon
                                            color="green"
                                            sx={{
                                                width: "10px",
                                                height: "10px",
                                                mr: 1,
                                            }}
                                        />
                                        <Typography sx={{ fontSize: ".9rem", fontWeight: 500 }}>MA 26100</Typography>
                                    </Grid2>
                                    <Grid2 container>
                                        <Chip
                                            label="4"
                                            variant="outlined"
                                            color="unselectedText"
                                            size="small"
                                            sx={{
                                                fontWeight: "700",
                                                fontSize: ".7rem",
                                                color: "selectedText.main",
                                                mr: 1
                                            }}
                                        />
                                        <Chip
                                            label="A+"
                                            variant="outlined"
                                            color="unselectedText"
                                            size="small"
                                            sx={{
                                                fontWeight: "700",
                                                fontSize: ".7rem",
                                                color: "selectedText.main",
                                            }}
                                        />

                                    </Grid2>

                                </Grid2>


                                <Grid2 container sx={{
                                    display: "flex",
                                    flexDirection: "row",
                                    justifyContent: "space-between",
                                    backgroundColor: "#FFF",
                                    p: 1.5,
                                    m: 1.5,
                                    flexGrow: 1,
                                    borderRadius: "6px",
                                }}>

                                    <Grid2 container sx={{
                                        display: "flex",
                                        alignItems: "center",
                                    }}>

                                        <Brightness1Icon
                                            color="green"
                                            sx={{
                                                width: "10px",
                                                height: "10px",
                                                mr: 1,
                                            }}
                                        />
                                        <Typography sx={{ fontSize: ".9rem", fontWeight: 500 }}>MA 26100</Typography>
                                    </Grid2>
                                    <Grid2 container>
                                        <Chip
                                            label="4"
                                            variant="outlined"
                                            color="unselectedText"
                                            size="small"
                                            sx={{
                                                fontWeight: "700",
                                                fontSize: ".7rem",
                                                color: "selectedText.main",
                                                mr: 1
                                            }}
                                        />
                                        <Chip
                                            label="A+"
                                            variant="outlined"
                                            color="unselectedText"
                                            size="small"
                                            sx={{
                                                fontWeight: "700",
                                                fontSize: ".7rem",
                                                color: "selectedText.main",
                                            }}
                                        />

                                    </Grid2>

                                </Grid2>

                            </Grid2>

                        </Grid2>
                    </Grid2>
                </Grid2>
                <Typography variant="h1">This pathfinder</Typography>
                <Typography variant="h4">Sup</Typography>
                <Button
                    color="yellow"
                    variant="contained"
                >
                    Hello from MUI v5
                </Button>
            </div>
        </ThemeProvider >
    )
}
