import React from "react";
import { Link } from "react-router-dom";
import "./Header.scss";

class Header extends React.Component {
    render() {
        return (
            <header className="AppHeader">
                <h1>Philes.rs</h1>
                <div className="NavBar">
                    <div className="NavItem">
                        <Link to="/">Upload</Link>
                    </div>
                    <div className="NavItem">
                        <Link to="/browse">Browse</Link>
                    </div>
                    <div className="NavItem">
                        <Link to="/about">About</Link>
                    </div>
                </div>
            </header>
        );
    }
}

export default Header;
