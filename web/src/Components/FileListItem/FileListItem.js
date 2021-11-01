import React from "react";
import DeleteIcon from "./delete.svg";
import "./FileListItem.scss";

class FileListItem extends React.Component {
    constructor(props) {
        super();
        this.state = {
            keyValue: props.keyValue,
        };
    }

    render() {
        return (
            <div className="FileListItem">
                <img
                    className="DeleteIcon UploadAreaContainer-pointerActive"
                    src={DeleteIcon}
                    alt="Remove File"
                    onClick={this.handleDeleteClick}
                />
                <p className="FileTitle">{this.props.name}</p>
            </div>
        );
    }

    handleDeleteClick = () => {
        this.props.removeFile(this.state.keyValue);
    };
}

export default FileListItem;
