var generate_table = async () => {
    // get the reference for the body

    // creates a <table> element and a <tbody> element
    let tbl = document.createElement("table");
    let tblBody = document.createElement("tbody");
    let tblHead = document.createElement("thead");

    let headRow = document.createElement("tr");

    let idHeader = document.createElement("th");
    let idText = document.createTextNode("Id");
    idHeader.append(idText);

    let urlHeader = document.createElement("th");
    let urlText = document.createTextNode("Link");
    urlHeader.append(urlText);

    headRow.appendChild(idHeader);
    headRow.appendChild(urlHeader);
    tblHead.appendChild(headRow);

    let resp = await fetch("/files");
    let data = await resp.json();

    // creating all cells
    for (let f of data) {
        // creates a table row
        let row = document.createElement("tr");

        let id = document.createElement("td");
        let idText = document.createTextNode(f.id);
        id.append(idText);

        let link = document.createElement("td");
        let linkAnchor = document.createElement("a");
        let linkText = document.createTextNode("Link to Image");
        
        linkAnchor.title = "Link to Image";
        linkAnchor.href = "/file/" + f.location;
        linkAnchor.appendChild(linkText);
        link.appendChild(linkAnchor);


        row.appendChild(id);
        row.appendChild(link);
        // add the row to the end of the table body
        tblBody.appendChild(row);
    }

    tbl.appendChild(tblHead);
    // put the <tbody> in the <table>
    tbl.appendChild(tblBody);
    // appends <table> into <body>
    var body = document.getElementsByTagName("body")[0];
    body.appendChild(tbl);
    // sets the border attribute of tbl to 2;
    tbl.setAttribute("border", "2");
};

(async () => {
    await generate_table();
})();
