import './App.css';
import { Typography, Button } from '@mui/material';

function App() {
    return (
        <div className="App">
            <Typography variant="myVariant">This is my app</Typography>
            <Typography variant="h4">Sup</Typography>
            <Button
                color="secondary"
                variant="contained"
            >
                Hello from MUI v5
            </Button>
        </div>
    );
}

export default App;
