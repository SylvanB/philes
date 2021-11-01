import React from "react";
import "./Button.scss";

class Button extends React.Component {
    render() {
        return (
            <div className="button" onClick={this.props.handleOnClick}>
                {this.props.value}
            </div>
        );
    }
}

export default Button;
