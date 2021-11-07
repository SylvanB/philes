import React from "react";
import "../pageBase.scss";

class About extends React.Component {
    render() {
        return (
            <div className="bodyContainer">
                <h2>About Philes.rs</h2>
                <p>Philes.rs is a free, open source, file sharing service.</p>
                <p>
                    Created by <a href="https://sylvanb.dev">Sylvan Bowdler</a>
                </p>
            </div>
        );
    }
}

export default About;
