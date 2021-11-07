import React from "react";

class FileRow extends React.Component {
    render() {
        return (
            <tr>
                <td>{this.props.name}</td>
                <td>{this.props.location}</td>
            </tr>
        );
    }
}

export default FileRow;
