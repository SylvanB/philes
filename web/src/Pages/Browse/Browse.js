import React from "react";
import BrowseTable from "../../Components/BrowseTable/BrowseTable";
import FileApi from "../../Services/FileApi";

class Browse extends React.Component {
    state = {
        files: [],
    };

    render() {
        return (
            <div className="bodyContainer">
                <h2>Browse</h2>
                <BrowseTable files={this.state.files} />
            </div>
        );
    }

    async componentDidMount() {
        await this.getFiles();
    }

    async getFiles() {
        const fileApi = new FileApi();

        const files = await fileApi.getAll();

        console.log(files);

        this.setState({
            files: files,
        });
    }
}

export default Browse;
