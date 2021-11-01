import React from "react";
import "./UploadArea.scss";

class UploadArea extends React.Component {
    dropRef = React.createRef();
    inputOpenFileRef = React.createRef();

    state = {
        dragging: false,
        dragCount: 0,
    };

    render() {
        let style = "UploadAreaContainer";
        if (this.state.dragging) {
            style += " UploadAreaContainer-drag";
        }

        if (!this.props.isActive) {
            style += " UploadAreaContainer-pointerActive";
        }

        return (
            <div
                onClick={() => !this.props.isActive && this.showOpenFileDlg()}
                onChange={this.updateFiles}
                className={style}
                ref={this.dropRef}>
                <input
                    hidden
                    ref={this.inputOpenFileRef}
                    type="file"
                    multiple={true}></input>
                {this.props.children}
            </div>
        );
    }

    updateFiles = () => {
        if (this.inputOpenFileRef.current.files?.length > 0) {
            const files = [...this.inputOpenFileRef.current.files]
                .filter(
                    (f) => !f.name.endsWith(".exe") || f.size > 1024 * 1024 * 40
                )
                .slice(0, 5);

            this.props.updateFileState(files);
        }
    };

    showOpenFileDlg = () => {
        this.inputOpenFileRef.current.click();
    };

    componentDidMount() {
        let div = this.dropRef.current;
        this.dragCount = 0;

        div.addEventListener("dragenter", this.handleDragIn);
        div.addEventListener("dragleave", this.handleDragOut);
        div.addEventListener("dragover", this.handleDrag);
        div.addEventListener("drop", this.handleDrop);
    }

    componentWillUnmount() {
        let div = this.dropRef.current;

        div.removeEventListener("dragenter", this.handleDragIn);
        div.removeEventListener("dragleave", this.handleDragOut);
        div.removeEventListener("dragover", this.handleDrag);
        div.removeEventListener("drop", this.handleDrop);
    }

    handleDrag = (e) => {
        e.preventDefault();
        e.stopPropagation();
    };

    handleDragIn = (e) => {
        e.preventDefault();
        e.stopPropagation();

        this.dragCount += 1;
        if (e.dataTransfer.items?.length > 0) {
            this.setState({ dragging: true });
        }
    };

    handleDragOut = (e) => {
        e.preventDefault();
        e.stopPropagation();

        this.dragCount -= 1;
        if (this.dragCount > 0) return;
        this.setState({ dragging: false });
    };

    handleDrop = (e) => {
        e.preventDefault();
        e.stopPropagation();

        this.setState({ dragging: false });
        if (e.dataTransfer.files?.length > 0) {
            this.props.updateFileState(e.dataTransfer.files);
        }
        this.dragCount = 0;
    };
}

export default UploadArea;
