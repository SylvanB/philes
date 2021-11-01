import React from "react";
import "./App.scss";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import Upload from "./Pages/Upload/Upload";
import Header from "./Components/Header/Header";
import Browse from "./Pages/Browse/Browse";
import About from "./Pages/About/About";

class App extends React.Component {
    render() {
        return (
            <Router>
                <div className="App">
                    <Header />
                    <Switch>
                        <Route path="/" exact component={Upload} />
                        <Route path="/browse" exact component={Browse} />
                        <Route path="/about" exact component={About} />
                    </Switch>
                </div>
            </Router>
        );
    }
}

export default App;
