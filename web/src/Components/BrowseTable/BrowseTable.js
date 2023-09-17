import React from "react";
import FileRow from "./FileRow";

class BrowseTable extends React.Component {
    render() {
        return (
            <table>
               <thead>
                   <tr>
                       <th>Filename</th>
                       <th>File Location</th>
                   </tr>
               </thead>
                <tbody>
                    {Object.entries(this.props.files).map((file) => {
                        return <FileRow name={file[0]} location={file[1]} />;
                    })}
                </tbody>
            </table>
        );
    }
}

export default BrowseTable;
