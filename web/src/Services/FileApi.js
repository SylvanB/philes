class FileApi {
    async getById(id) {
        return {};
    }

    async getAll() {
        return [{}, {}, {}];
    }

    async deleteById(id) {
        return true;
    }

    async upsert(newFiles) {
        // Use fetch w/ each file to send off to backend.

        let formData = new FormData();

        for (const file in newFiles) {
            formData.append(`file${file}`, newFiles[file]);
        }

        let req = new Request("http://localhost:8000/files", {
            method: "POST",
            body: formData,
        });

        let uploadedFiles = await fetch(req)
            .then(async (resp) => {
                return await resp.json();
            })
            .catch((err) => {
                return false;
            });

        console.log(uploadedFiles);
        return uploadedFiles;
    }
}

export default FileApi;
