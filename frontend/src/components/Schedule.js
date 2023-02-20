import {useState} from 'react';

const Schedule = () => {
    const [degree, setDegree] = useState('');
    //axios.get(`http://127.0.0.1:8080/schedule/` + degree)
    /*.then(res => {
        console.log(res);
        //Show this on a router page
    })
    console.log("degrees: ", degrees);
    //Grab the value of the */

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