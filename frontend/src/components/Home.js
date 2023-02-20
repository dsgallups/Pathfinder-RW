import * as React from 'react';
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import Button from "@mui/material/Button";
import { useState, useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';


// Add this in your component file
require('react-dom');
window.React2 = require('react');
console.log(window.React1 === window.React2);

const HomeButton = ({degree}) => {
    const navigate = useNavigate;

    function buttonClick() {
        navigate("/schedule/" + degree);
    }

    return <Button variant="contained" onClick={buttonClick}>Submit</Button>


}
const Home = () => {
    const [degree, setDegree] = React.useState('');
    const [degrees, setDegrees] = useState([]);


    useEffect(() => {
        axios.get(`http://127.0.0.1:8080/degrees`)
            .then(res => {
                const retdegrees = res.data;
                //<MenuItem value={degree.code}>{degree.code}</MenuItem>
                //setDegrees(retdegrees.map(degree => degree.code));
                setDegrees(retdegrees.map(degree => <MenuItem value={degree.code}>{degree.code}</MenuItem>));
                console.log("response: ", retdegrees);
                console.log("degrees: ", degrees);
            })
    }, []);

    const handleChange = (e) => {
      setDegree(e.target.value);
    };
  
    return (
      <Box sx={{
        width: 600,
        height: 300,
        margin: "0 auto",
        marginTop: 20,
      }}>
        <div>Select the Degree for which you want to schedule</div>
        <FormControl 
            fullWidth 
            color="primary"
            margin="normal"
        >
          <InputLabel id="degree-input-select">Degree</InputLabel>
          <Select
            labelId="degree-input-select"
            id="degree-select"
            value={degree}
            label="Select Degree"
            onChange={handleChange}
          >
            {degrees}
          </Select>
          <HomeButton degree={degree}/>
        </FormControl>
      </Box>
    );
}

export default Home;