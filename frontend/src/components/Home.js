import * as React from 'react';
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import Button from "@mui/material/Button";
const Home = () => {
    const [age, setAge] = React.useState('');

    const handleChange = (e) => {
      setAge(e.target.value);
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
            value={age}
            label="Age"
            onChange={handleChange}
          >
            <MenuItem value={10}>Ten</MenuItem>
            <MenuItem value={20}>Twenty</MenuItem>
            <MenuItem value={30}>Thirty</MenuItem>
          </Select>
          <Button variant="contained">Submit</Button>
        </FormControl>
      </Box>
    );
}

export default Home;