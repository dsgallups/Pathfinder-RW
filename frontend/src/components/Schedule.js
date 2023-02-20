import {useState, useEffect} from 'react';
import { useSearchParams, useParams } from 'react-router-dom';
import axios from 'axios';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { ThemeOptions } from '@mui/material/styles';
import { Box, Container } from '@mui/system';
import { CssBaseline, Typography, Stack, List, ListItem, ListItemText } from '@mui/material';
import Grid2 from '@mui/material/Unstable_Grid2'; // Grid version 2


const theme = createTheme({

    palette: {
        mode: 'dark',
        primary: { main: 'rgb(102, 157, 246)' },
        background: { paper: 'rgb(5, 30, 52)' },
    }
});

const ScheduleTitle = ({degree}) => {
    return <Typography component="h1" variant="h5">{degree.name}</Typography>
};

const Schedule = (props) => {
    const [searchParams, setSearchParams] = useSearchParams();
    let [degree, setDegree] = useState();
    let [schedule, setSchedule] = useState();
    let {code} = useParams();

    useEffect(() => {

        axios.get('http://127.0.0.1:8080/degree/' + code)
            .then((res) => {
                console.log("degree: ", res.data);
                setDegree(<Typography component="h1" variant="h5">{res.data.name}</Typography>);

                axios.get(`http://127.0.0.1:8080/schedule/` + code)
                .then((res) => {
                    console.log(res.data);
                    setSchedule(res.data.periods.map(period => {
                        return (
                            <Grid2 xs={5}>
                                <Typography>{period.time} {period.year}</Typography>
                                <List sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
                                    {period.classes.map(c => {
                                        return (
                                            <ListItem key={c.id}>
                                                <ListItemText primary={c.name}/>
                                            </ListItem>
                                        )
                                        
                                    })}
                                </List>
                            </Grid2>
                        )
                    }))
                })

            })
    }, [])

    //axios.get(`http://127.0.0.1:8080/schedule/` + degree)
    /*.then(res => {
        console.log(res);
        //Show this on a router page
    })
    console.log("degrees: ", degrees);
    //Grab the value of the */

    return (
        <ThemeProvider theme={theme}>
            <Container component="main" maxWidth="lg">
                <CssBaseline/>
                <Box
                    sx={{
                        marginTop: 8,
                        display: 'flex',
                        flexDirection: 'column',
                        alignItems: 'center',
                    }}
                >
                    {degree}
                    <Grid2 
                        container 
                        sx={{
                            width: "90%",
                            justifyContent: "center"
                        }}
                        spacing={2}
                    >
                        {schedule}
                    </Grid2>
                </Box>
            </Container>
        </ThemeProvider>
    )
}

export default Schedule;