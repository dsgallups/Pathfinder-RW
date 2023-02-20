import {useState} from 'react';

const Schedule = () => {
    const [degree, setDegree] = useState('');

    return (
        <>

        <div className="Home centerContainer">
            <div className="homeHeader centerHeader">Bored at Purdue?</div>
            <div className="homeSubHeader centerSubHeader">join a local chat, create a study group, or find a game to play with other students</div>

        </div>
        </>
    )
}

export default Schedule;