import React from "react";
import "./Upload.scss";
import UploadArea from "../../Components/UploadArea/UploadArea";
import FileApi from "../../Services/FileApi";
import Button from "../../Components/Button/Button";
import FileListItem from "../../Components/FileListItem/FileListItem";

class Upload extends React.Component {
    fileApi = new FileApi();

    state = {
        files: [],
    };

    render() {
        return (
            <div className="uploadPageContainer">
                <h2>Upload</h2>
                <input
                    hidden
                    type="file"
                    id="fileInput"
                    ref={this.inputOpenFileRef}
                />
                <UploadArea
                    isActive={this.state.files.length > 0}
                    updateFileState={this.updateFileState}>
                    <div className="FileList">{this.getFileListContent()}</div>
                </UploadArea>
                <Button value="Upload" handleOnClick={this.uploadFiles} />
            </div>
        );
    }

    updateFileState = (files) => {
        const filesArr = [...files];
        this.setState({ files: filesArr });
    };

    removeFile = (fileName) => {
        const updatedFileList = [...this.state.files].filter(
            (x) => x.name !== fileName
        );

        this.setState({ files: updatedFileList });
    };

    getFileListContent = () => {
        if (this.state.files?.length <= 0) {
            return (
                <p style={{ alignSelf: "center" }}>
                    Drag the file(s) you want to upload here, or click here to
                    choose a file.
                </p>
            );
        }

        let listItems = [];
        for (const file of this.state.files) {
            listItems.push(
                <FileListItem
                    key={file.name}
                    keyValue={file.name}
                    removeFile={this.removeFile}
                    name={file.name}
                />
            );
        }
        return listItems;
    };

    uploadFiles = async () => {
        if (this.state.files?.length > 0) {
            await this.fileApi.upsert(this.state.files);
            this.setState({
                files: [],
            });
        }
    };
}

export default Upload;
